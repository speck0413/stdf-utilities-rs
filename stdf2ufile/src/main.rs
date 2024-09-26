use std::{collections::HashMap, fs::File, io::Write};

use argparse::{ArgumentParser, Store};
use stdf_reader::{StdfParser, StdfRecord, V1};

// Define a struct to hold the arguments
struct Arguments {
    stdf_filename: String,
    output_filename: String,
}

fn get_cstr_format(fmt: &Option<String>, val: &Option<f32>) -> String {
    match val {
        Some(llm) => {
            let mut ret_val = format!("{}", llm);

            if fmt.is_some() && fmt.clone().unwrap().len() > 0 {
                if let Ok(s) = sprintf::sprintf!(fmt.clone().unwrap().as_str(), llm.to_owned()) { ret_val = s; }
            }
            // if let Some(fmt) = fmt {
            //     if fmt.len() > 0 {
            //         if let Ok(s) = sprintf::sprintf!(fmt.as_str(), llm) { ret_val = s; }
            //     }
            // }

            ret_val
        }
        None => "".to_string()
    }
}

fn rec_to_ufile_line(site_to_part_idx: &HashMap<u8, u32>, rec: &StdfRecord) -> Option<String> {
    let fail_type_regex = regex::Regex::new(r"S[0-9]+_").unwrap();

    // do the thing
    match rec {
        // For all other record types, do nothing
        StdfRecord::DTR(rec) => {
            Some(format!("{}", rec.text_dat.to_owned()))
        },
        // For all other record types, do nothing
        StdfRecord::PTR(rec) => {
            // add to log
            let part_idx = site_to_part_idx.get(&rec.site_num).unwrap_or(&0);
            let llm = get_cstr_format(&rec.c_llmfmt, &rec.lo_limit);
            let hlm = get_cstr_format(&rec.c_hlmfmt, &rec.hi_limit);
            let result = get_cstr_format(&rec.c_resfmt, &Some(rec.result));
            let units = if let Some(units) = rec.units.to_owned() { units } else { String::new() };
            let units = if units.len() > 0 { format!("({})", units) } else { "".to_string() };
            let failed_string = if fail_type_regex.is_match(&rec.test_txt) { "Failed  " } else { "failed  " };

            let mut text = if rec.test_flg[0] & 0b01011100 == 0 && rec.test_flg[0] & 0b10000000 != 0 {
                failed_string.to_string()
            } else {
                "".to_string()
            };
            let llm_cmp = if !llm.is_empty() { if rec.parm_flg[0] & 0x40 != 0 { " <= " } else { " < " }} else {""};
            let hlm_cmp = if !hlm.is_empty() { if rec.parm_flg[0] & 0x80 != 0 { " <= " } else { " < " }} else {""};
            text = format!("{:04}  {}{}  {}{}{}{}{} {}\n", part_idx, text, rec.test_txt, llm, llm_cmp, result, hlm_cmp, hlm, units);

            Some(text)
        },
        StdfRecord::FTR(rec) => {
            // add to log
            let part_idx = site_to_part_idx.get(&rec.site_num).unwrap_or(&0);
            let opt_flag = rec.opt_flag[0];
            let test_flag = rec.test_flg[0];
            let mut text = "".to_string();
            let failed_string = if fail_type_regex.is_match(&rec.test_txt) { "Failed  " } else { "failed  " };

            if opt_flag & 0x08 == 0 {
                // return num_fail
                if rec.num_fail > 0 {
                    text = failed_string.to_string();
                }
            } else {
                // no num_fail information
                if test_flag & 0x54 == 0 && test_flag != 0 {
                    // test failed, but no num_fail information
                    text = failed_string.to_string();
                }
            };

            text = format!("{:04}  {}{}  {}  {}\n", part_idx, text, rec.vect_nam, rec.num_fail, rec.test_txt);
            Some(text)
        },
        StdfRecord::STR(rec) => {
            // add to log
            let part_idx = site_to_part_idx.get(&rec.site_num).unwrap_or(&0);
            let text = format!("{:04}  {:?}\n", part_idx, rec);
            Some(text)
        },
        StdfRecord::GDR(rec) => {
            let text: String = if let Some(V1::Cn(usr_type)) = rec.gen_data.get(0)  {
                match usr_type.as_str() {
                    "SHMOO" => {
                        let mut text = format!("SHMOO BEGIN\n");
                        for (i, val) in rec.gen_data.iter().enumerate() {
                            if i == 0 { continue; }
                            if let V1::Cn(data) = val {
                                text = format!("{}{}\n", text, data);
                            }
                        }
                        let text = format!("{}SHMOO END\n", text);
                        text
                    },
                    _ => {
                        // unhandled, just print it out
                        format!("{}\n", format!("{:?}\n", rec))
                    }
                }
            } else {
                // unhandled type, just print it out
                format!("{:?}\n", rec)
            };
            Some(text)
        }
        _ => { None }
    }
}

fn main() {
    // Call the function to parse the arguments
    let args = parse_arguments();
    
    let mut parser = StdfParser::new(&args.stdf_filename, &None).unwrap();
    let mut site_to_part_idx = HashMap::<u8, u32>::new();
    let mut part_idx = 1;
    let mut out_f = File::create(args.output_filename).expect("Unable to create file");

    loop {
        if let Some(rec) = parser.next() {
            // Loop until a record isn't found
            match rec {
                // Do nothing if the record is a MRR
                Ok((StdfRecord::MRR(_), _)) => {
                    break;
                },
                // If the record is a PIR, update the site_to_part_idx map
                Ok((StdfRecord::PIR(rec), _)) => {
                    // update site_to_part_idx
                    let site_num = rec.site_num;
                    site_to_part_idx.insert(site_num, part_idx);
                    part_idx = part_idx + 1;
                },
                // All other records need to get formatted for the ufile
                Ok((rec, _)) => {
                    // add to log
                    if let Some(msg) = rec_to_ufile_line(&site_to_part_idx, &rec) {
                        out_f.write(msg.as_bytes()).expect("Unable to write data");
                    }
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                },
            }
        } else {
            // we have gotten all the records
            break;
        }
    }
}

// Function to parse the arguments
fn parse_arguments() -> Arguments {
    // Create a mutable vector to store the STDF filenames
    let mut args = Arguments { stdf_filename: String::new(), output_filename: String::new() };

    // Create ArgumentParser variable
    let mut ap = ArgumentParser::new();

    // Set the application description
    ap.set_description("Takes an STDF and converts it to a human readable text version");

    // Add all arguments and associated variables
    // Here we are adding an argument for the STDF input file
    ap.refer(&mut args.stdf_filename)
    .add_argument("Stdf Input", Store, "Stdf input file to be converted").required();

    ap.refer(&mut args.output_filename)
    .add_argument("Ufile Output Filename", Store, "Ufile output location.");

    // Parse the arguments and store them
    ap.parse_args_or_exit();

    // Drop the ArgumentParser so we can return the arguments
    std::mem::drop(ap);

    // Return the arguments in a struct
    args
}
