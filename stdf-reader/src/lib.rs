use std::{collections::{BTreeMap, HashMap}, fs::File, io::{BufReader, Write}};
use sprintf::sprintf;

mod rec_to_string;
pub mod stdf_parser;

pub use stdf_parser::*;

use polars;
use regex::Regex;
use rust_stdf::{stdf_file::StdfReader, *};
use const_crc32::crc32;

#[macro_use]
extern crate ini;

#[derive(Debug, Clone)]
pub struct DtrConfiguration {
    name: String,
    regex: Regex,
    id_fmt: String,
    link_to_records: Vec<String>,
    text_fmt: String,
    clear_on_prr: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DtrInfo {
    pub uuid: String,
    pub id: String,
    pub inject_into: Vec<String>,
    pub text: String,
    pub clear_on_prr: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StdfInfo {
    pub site_number: u8,
    pub hard_bin: u16,
    pub soft_bin: u16,
    pub test_time: Vec<u32>,
    pub test_count: Vec<u16>,
}

pub struct FirstPassInfo {
    min_site_num: u8,
    part_ids: Vec<String>,
    dtr_info: Vec<DtrInfo>,
}

pub fn load_dtr_config(config_fname: &Option<String>) -> Vec<DtrConfiguration> {
    let mut ret_val = Vec::<DtrConfiguration>::new();

    if let Some(config_fname) = config_fname {
        // grab dtr configuration
        let dtr_cfg_dict =  ini!(config_fname.as_str());

        for (key, val) in dtr_cfg_dict {
            ret_val.push(DtrConfiguration {
                name: key.to_owned(),
                regex: Regex::new(val["regex"].to_owned().unwrap_or("".to_string()).as_str()).expect(format!("Error while parsing regex in section {} in dtr configuration file.", key).as_str()),
                id_fmt: val["id_fmt"].to_owned().unwrap_or("".into()),
                link_to_records: val["link_to_records"].to_owned().unwrap_or("".to_string()).split(",").into_iter().map(|s| s.to_ascii_uppercase().into()).collect(),
                text_fmt: val["text_fmt"].to_owned().unwrap_or("".to_string()),
                clear_on_prr: val["clear_on_prr"].to_owned().unwrap_or("False".into()).eq_ignore_ascii_case("true")
            })
        }
    }

    return ret_val;
}

pub fn parse_dtr(rec: &DTR, dtr_cfg_dict: &Vec<DtrConfiguration>) -> Option<DtrInfo> {
    // Look through DTR configuration passed in and find matches to regex
    for val in dtr_cfg_dict {

        // check if matching regex
        if val.regex.is_match(&rec.text_dat) {
            
            // we have a match, handle it

            // format the id, or return default
            let id = if val.id_fmt.is_empty() {
                rec.text_dat.to_owned()
            } else {
                val.regex.replace(&rec.text_dat, val.id_fmt.to_owned()).into_owned()
            };

            // format the text or return default
            let text = if val.text_fmt.is_empty() {
                rec.text_dat.to_owned()
            } else {
                val.regex.replace(&rec.text_dat, val.text_fmt.to_owned()).into_owned()
            };

            // compute crc32 of dtr
            let uuid = crc32(rec.text_dat.as_bytes());

            // construct the return value
            return Some(DtrInfo {
                uuid: format!("{:x}", uuid),
                id,
                inject_into: val.link_to_records.to_owned(),
                text,
                clear_on_prr: val.clear_on_prr,
            });
        }
    }

    return None;
}

fn vec_mean<T: Ord+Clone>(numbers: &Vec<T>) -> f64 
where
    f64: std::convert::From<T> {

    // summation will be done as f32
    let mut sum: f64 = 0.0;

    // sum all numbers
    for num in numbers {
        sum = sum + f64::from(num.clone());
    }

    // return average
    sum / numbers.len() as f64
}

fn vec_median<T: Ord+Clone>(numbers: &Vec<T>) -> T {
    // sort numbers
    let mut numbers = numbers.clone();
    numbers.sort();

    // find middle point in array
    let mid = numbers.len() / 2;

    // return middle point
    numbers[mid].clone()
}

fn vec_min<T: Ord+Clone>(numbers: &Vec<T>) -> T {
    // sort numbers
    let mut numbers = numbers.clone();
    numbers.sort();

    // return middle point
    numbers.first().unwrap().clone()
}

fn vec_max<T: Ord+Clone>(numbers: &Vec<T>) -> T {
    // sort numbers
    let mut numbers = numbers.clone();
    numbers.sort();

    // return middle point
    numbers.last().unwrap().clone()
}

fn handle_mpr_defaults(rec: &MPR, test_defaults_mpr: &mut HashMap<u32, MPR>) -> MPR {
    // clone the MPR record for manupulation
    let mut rec = rec.clone();

    if rec.opt_flag != None && rec.opt_flag.unwrap()[0] & 0x40 != 0 { rec.lo_limit = None; rec.llm_scal = None; }
    if rec.opt_flag != None && rec.opt_flag.unwrap()[0] & 0x80 != 0 { rec.hi_limit = None; rec.hlm_scal = None; }
    
    if let Some(defaults) = test_defaults_mpr.get(&rec.test_num) {
        // We have access to the defaults, update what needs updating
        if rec.opt_flag == None { rec.opt_flag = defaults.opt_flag.to_owned(); }
        if rec.res_scal == None { rec.res_scal = defaults.res_scal.to_owned(); }
        if rec.llm_scal == None { rec.llm_scal = defaults.llm_scal.to_owned(); }
        if rec.hlm_scal == None { rec.hlm_scal = defaults.hlm_scal.to_owned(); }
        if rec.lo_limit == None { rec.lo_limit = defaults.lo_limit.to_owned(); }
        if rec.hi_limit == None { rec.hi_limit = defaults.hi_limit.to_owned(); }
        if rec.start_in == None { rec.start_in = defaults.start_in.to_owned(); }
        if rec.incr_in  == None { rec.incr_in  = defaults.incr_in.to_owned();  }
        if rec.rtn_indx == None { rec.rtn_indx = defaults.rtn_indx.to_owned(); }
        if rec.units    == None { rec.units    = defaults.units.to_owned();    }
        if rec.units_in == None { rec.units_in = defaults.units_in.to_owned(); }
        if rec.c_resfmt == None { rec.c_resfmt = defaults.c_resfmt.to_owned(); }
        if rec.c_llmfmt == None { rec.c_llmfmt = defaults.c_llmfmt.to_owned(); }
        if rec.c_hlmfmt == None { rec.c_hlmfmt = defaults.c_hlmfmt.to_owned(); }
        if rec.lo_spec  == None { rec.lo_spec  = defaults.lo_spec.to_owned();  }
        if rec.hi_spec  == None { rec.hi_spec  = defaults.hi_spec.to_owned();  }
    } else {
        // don't have the updates, store the record for later use
        test_defaults_mpr.insert(rec.test_num, rec.clone());
    }

    rec
}

fn handle_ptr_defaults(rec: &PTR, test_defaults_ptr: &mut HashMap<u32, PTR>) -> PTR {
    // clone the PTR record for manupulation
    let mut rec = rec.clone();

    if rec.opt_flag != None && rec.opt_flag.unwrap()[0] & 0x40 != 0 { rec.lo_limit = None; rec.llm_scal = None; }
    if rec.opt_flag != None && rec.opt_flag.unwrap()[0] & 0x80 != 0 { rec.hi_limit = None; rec.hlm_scal = None; }

    if let Some(defaults) = test_defaults_ptr.get(&rec.test_num) {
        // We have access to the defaults, update what needs updating
        if rec.opt_flag == None { rec.opt_flag = defaults.opt_flag.to_owned(); }
        if rec.res_scal == None { rec.res_scal = defaults.res_scal.to_owned(); }
        if rec.llm_scal == None { rec.llm_scal = defaults.llm_scal.to_owned(); }
        if rec.hlm_scal == None { rec.hlm_scal = defaults.hlm_scal.to_owned(); }
        if rec.lo_limit == None { rec.lo_limit = defaults.lo_limit.to_owned(); }
        if rec.hi_limit == None { rec.hi_limit = defaults.hi_limit.to_owned(); }
        if rec.units    == None { rec.units    = defaults.units.to_owned();    }
        if rec.c_resfmt == None { rec.c_resfmt = defaults.c_resfmt.to_owned(); }
        if rec.c_llmfmt == None { rec.c_llmfmt = defaults.c_llmfmt.to_owned(); }
        if rec.c_hlmfmt == None { rec.c_hlmfmt = defaults.c_hlmfmt.to_owned(); }
        if rec.lo_spec  == None { rec.lo_spec  = defaults.lo_spec.to_owned();  }
        if rec.hi_spec  == None { rec.hi_spec  = defaults.hi_spec.to_owned();  }
    } else {
        // don't have the updates, store the record for later use
        test_defaults_ptr.insert(rec.test_num, rec.clone());
    }

    rec
}

fn handle_ftr_defaults(rec: &FTR, test_defaults_ftr: &mut HashMap<u32, FTR>) -> FTR {
    // clone the PTR record for manupulation
    let mut rec = rec.clone();

    if let Some(defaults) = test_defaults_ftr.get(&rec.test_num) {
        // We have access to the defaults, update what needs updating
        if rec.opt_flag[0] & 0x01 > 0 { rec.cycl_cnt = defaults.cycl_cnt.to_owned(); }
        if rec.opt_flag[0] & 0x02 > 0 { rec.rel_vadr = defaults.rel_vadr.to_owned(); }
        if rec.opt_flag[0] & 0x04 > 0 { rec.rept_cnt = defaults.rept_cnt.to_owned(); }
        if rec.opt_flag[0] & 0x08 > 0 { rec.num_fail = defaults.num_fail.to_owned(); }
        if rec.opt_flag[0] & 0x10 > 0 { rec.xfail_ad = defaults.xfail_ad.to_owned(); }
        if rec.opt_flag[0] & 0x10 > 0 { rec.yfail_ad = defaults.yfail_ad.to_owned(); }
        if rec.opt_flag[0] & 0x20 > 0 { rec.vect_off = defaults.vect_off.to_owned(); }
        if rec.rtn_icnt == 0          { rec.rtn_indx = defaults.rtn_indx.to_owned(); }
        if rec.rtn_icnt == 0          { rec.rtn_stat = defaults.rtn_stat.to_owned(); }
        if rec.pgm_icnt == 0          { rec.pgm_indx = defaults.pgm_indx.to_owned(); }
        if rec.pgm_icnt == 0          { rec.pgm_stat = defaults.pgm_stat.to_owned(); }
        if rec.fail_pin.len() == 0    { rec.fail_pin = defaults.fail_pin.to_owned(); }
        if rec.vect_nam.len() == 0    { rec.vect_nam = defaults.vect_nam.to_owned(); }
        if rec.time_set.len() == 0    { rec.time_set = defaults.time_set.to_owned(); }
        if rec.op_code.len() == 0     { rec.op_code  = defaults.op_code.to_owned();  }
        if rec.test_txt.len() == 0    { rec.test_txt = defaults.test_txt.to_owned(); }
        if rec.alarm_id.len() == 0    { rec.alarm_id = defaults.alarm_id.to_owned(); }
        if rec.prog_txt.len() == 0    { rec.prog_txt = defaults.prog_txt.to_owned(); }
        if rec.rslt_txt.len() == 0    { rec.rslt_txt = defaults.rslt_txt.to_owned(); }
        if rec.patg_num == 255        { rec.patg_num = defaults.patg_num.to_owned(); }
        if rec.spin_map.len() == 0    { rec.spin_map = defaults.spin_map.to_owned(); }
    } else {
        // don't have the updates, store the record for later use
        test_defaults_ftr.insert(rec.test_num, rec.clone());
    }

    rec
}

//////////////////////////////////////////////////////////////////////
/// Description: Makes first pass thru the STDF finding all Test ID's and DTR ID's
//////////////////////////////////////////////////////////////////////
pub fn first_pass_stdf(stdf_path: &String, dtr_config: &Vec<DtrConfiguration>) -> Result<FirstPassInfo, String> {
    let mut min_site_num = 255u8;
    let mut part_ids = Vec::<String>::new();
    let mut dtr_info = Vec::<DtrInfo>::new();
    
    let mut reader = match StdfReader::new(&stdf_path) {
        // return if successful
        Ok(reader) => reader,

        // print full error and return the error message if not
        Err(err) => {
            println!("Error while loading stdf: {:?}\n", err);
            return Err(err.msg);
        }
    };

    
    for stdf_rec in reader.get_record_iter() {
        if let Ok(stdf_rec) = stdf_rec {
            match stdf_rec {
                // map all dtr id's for later
                StdfRecord::DTR(rec) => if let Some(l_dtr_info) = parse_dtr(&rec, &dtr_config) {
                    let mut contains_dtr = false;
                    for info in &dtr_info {
                        if info.id == l_dtr_info.id {
                            contains_dtr = true;
                            break;
                        }
                    }
                    if contains_dtr == false {
                        dtr_info.push(l_dtr_info.clone());
                    }
                },

                // map PRR test_ids based on when they're encountered
                StdfRecord::PRR(rec) => {
                    min_site_num = if min_site_num < rec.site_num { min_site_num } else { rec.site_num };
                    part_ids.push(rec.part_id);
                },

                // ignore most records
                _ => {},
            }
        }
    }

    Ok(FirstPassInfo { min_site_num, part_ids, dtr_info })
}

fn data_to_string(data: &Option<f32>, scale: &Option<i8>, format: &Option<String>, default: &String) -> String {
    // grab scale or default to 0
    let scale = scale.unwrap_or(0);

    // determine the format to use
    let format = format.clone().unwrap_or("%f".to_string());
    let format = if format == "" { "%f".into() } else { format };

    // format data using all information available.
    if let Some(data) = data {
        if data.is_finite() {
            sprintf!(format.as_str(), data * 10f32.powi(scale as i32)).unwrap_or(format!("{}", data * 10f32.powi(scale as i32))) //("Error Formatting Data".to_string())
        } else {
            default.clone()
        }
    } else {
        default.clone()
    }
}

pub fn convert_stdf2csv(stdf_path: &String, csv_path: &String, dtr_cfg_file: &Option<String>) -> Result<(), String> {
    // dictionary lookup table for default values
    let mut test_defaults_ptr = HashMap::<u32, PTR>::new();
    let mut test_defaults_mpr = HashMap::<u32, MPR>::new();
    let mut test_defaults_ftr = HashMap::<u32, FTR>::new();
    let dtr_config = load_dtr_config(dtr_cfg_file);
    let mut dtr_map = BTreeMap::<String, DtrInfo>::new();
    let mut dtr_id_in_col_idx_order = Vec::<String>::new();
    let csv_part_summary_path = csv_path.replace(".csv", ".part.summary.csv");
    let csv_stdf_summary_path = csv_path.replace(".csv", ".stdf.summary.csv");
    let csv_path = csv_path.replace(".csv", ".tests.csv");
    let mut stdf_summary_statistics = BTreeMap::<u16, BTreeMap<u16, BTreeMap<u8, StdfInfo>>>::new();
    let mut pmr_dict = BTreeMap::<u16, PMR>::new();

    // open csv files or error out
    let mut csv_file = std::fs::File::create(&csv_path).expect(format!("Error while trying to create the csv file {}.", &csv_path).as_str());
    let mut csv_part_summary_file = std::fs::File::create(&csv_part_summary_path).expect(format!("Error while trying to create the csv file {}.", &csv_part_summary_path).as_str());
    let mut csv_stdf_summary_file = std::fs::File::create(&csv_stdf_summary_path).expect(format!("Error while trying to create the csv file {}.", &csv_stdf_summary_path).as_str());

    // perform first pass through STDF to collect identifiers
    let mut first_pass_info = match first_pass_stdf(stdf_path, &dtr_config) {
        Ok(tuple) => tuple,
        Err(msg) => return Err(msg)
    };

    // open stdf file and start reading
    let mut reader = match StdfReader::new(&stdf_path) {
        // return if successful
        Ok(reader) => reader,

        // print full error and return the error message if not
        Err(err) => {
            println!("Error while loading stdf: {:?}\n", err);
            return Err(err.msg);
        }
    };

    /////////////////////////////////////////////////////////
    // Write out test header
    /////////////////////////////////////////////////////////
    csv_file.write(b"\"Part ID\",\"TNum\",\"SiteNum\",\"TestText\",\"Context\",\"Low Limit\",\"Result\",\"Hi Limit\"").expect("Error while trying to write to the csv file.");
    for info in first_pass_info.dtr_info {
        let id = info.id.clone();
        let inject_into = info.inject_into.clone();
        if inject_into.contains(&"PTR".into()) || inject_into.contains(&"MPR".into()) || inject_into.contains(&"FTR".into()) {
            // write the id out to the file
            csv_file.write(format!("\",\"{}", id).as_bytes()).expect("Error while trying to write to the csv file.");

            // add the id onto the map, this is something we care about tracking...
            dtr_map.insert(id.clone(), info.clone());
            dtr_id_in_col_idx_order.push(id.clone());
        }
    }
    csv_file.write(b"\n").expect("Error while trying to write to the csv file.");

    /////////////////////////////////////////////////////////
    // write out part summary header
    /////////////////////////////////////////////////////////
    csv_part_summary_file.write(b"\"Part ID\",\"SiteNum\",\"Test Time\",\"Hard Bin\",\"Soft Bin\",\"Test Count\",\"X\",\"Y\"\n").expect("Error while trying to write to the part summary csv file.");

    for stdf_rec in reader.get_record_iter() {
        if let Ok(stdf_rec) = stdf_rec {
            match stdf_rec {
                // Informational Record
                StdfRecord::DTR(rec) => if let Some(dtr_info) = parse_dtr(&rec, &dtr_config) {
                    dtr_map.insert(dtr_info.id.to_owned(), dtr_info.to_owned());
                },

                // Test Records
                StdfRecord::FTR(rec) => {
                    let part_id = first_pass_info.part_ids[usize::from(rec.site_num-first_pass_info.min_site_num)].clone();
                    let limit = if (rec.test_flg[0] & 0x40) != 0 { "" } else { "1" };
                    let result = if (rec.test_flg[0] & 0x40) != 0 { "" } else { if rec.test_flg[0] == 0 { "1" } else { "0" } };
                    let test_txt = rec.test_txt.replace("\"", "\"\"");
                    let context = rec.vect_nam.replace("\"", "\"\"");
                    let context = if context == "" { "".into() } else { "vect_name: ".to_string() + context.as_str() };
                    
                    let rec = handle_ftr_defaults(&rec, &mut test_defaults_ftr);

                    csv_file.write(format!("\"{}\",\"{}\",\"{}\",\"=\"\"{}\"\"\",\"=\"\"{}\"\"\",\"{}\",\"{}\",\"{}\"", part_id, rec.test_num, rec.site_num, test_txt, context, limit, result, limit).as_bytes()).expect("Error while trying to write to the csv file.");
                    for id in &dtr_id_in_col_idx_order {
                        csv_file.write(format!(",\"{}\"", dtr_map.get(id).unwrap().text).as_bytes()).expect("Error while trying to write to the csv file.");
                    }
                    csv_file.write(b"\n").expect("Error while trying to write to the csv file.");
                },

                StdfRecord::PTR(rec) => {
                    let part_id = first_pass_info.part_ids[usize::from(rec.site_num-first_pass_info.min_site_num)].clone();

                    let rec = handle_ptr_defaults(&rec, &mut test_defaults_ptr);

                    let lo_limit = data_to_string(&rec.lo_limit, &rec.llm_scal, &rec.c_llmfmt, &String::new());
                    let hi_limit = data_to_string(&rec.hi_limit, &rec.hlm_scal, &rec.c_hlmfmt, &String::new());
                    let result = data_to_string(&Some(rec.result), &rec.res_scal, &rec.c_resfmt, &String::new());
                    let test_txt = rec.test_txt.replace("\"", "\"\"").to_string();
                    let context = rec.units.unwrap_or("".into()).replace("\"", "\"\"");
                    let context = if context == "" { "".into() } else { "units: ".to_string() + context.as_str() };
                    
                    csv_file.write(format!("\"{}\",\"{}\",\"{}\",\"=\"\"{}\"\"\",\"=\"\"{}\"\"\",\"{}\",\"{}\",\"{}\"", part_id, rec.test_num, rec.site_num, test_txt, context, lo_limit, result, hi_limit).as_bytes()).expect("Error while trying to write to the csv file.");
                    for id in &dtr_id_in_col_idx_order {
                        csv_file.write(format!(",\"{}\"", dtr_map.get(id).unwrap().text).as_bytes()).expect("Error while trying to write to the csv file.");
                    }
                    csv_file.write(b"\n").expect("Error while trying to write to the csv file.");
                },

                StdfRecord::MPR(rec) => {
                    let part_id = first_pass_info.part_ids[usize::from(rec.site_num-first_pass_info.min_site_num)].clone();

                    let rec = handle_mpr_defaults(&rec, &mut test_defaults_mpr);
//
                    let lo_limit = data_to_string(&rec.lo_limit, &rec.llm_scal, &rec.c_llmfmt, &String::new());
                    let hi_limit = data_to_string(&rec.hi_limit, &rec.hlm_scal, &rec.c_hlmfmt, &String::new());
                    let test_txt = rec.test_txt.replace("\"", "\"\"").to_string();
                    let rtn_indx = rec.rtn_indx.unwrap_or(Vec::<u16>::new());
                    for i in 0..(rec.rtn_rslt.len()-1) {
                        let result = data_to_string(&Some(rec.rtn_rslt[i]), &rec.res_scal, &rec.c_resfmt, &String::new());
                        let pin_label = if pmr_dict[rtn_indx.get(i).unwrap()].log_nam == "" {
                            pmr_dict[rtn_indx.get(i).unwrap()].chan_nam.clone()
                        } else {
                            pmr_dict[rtn_indx.get(i).unwrap()].log_nam.clone()
                        };
                        // let pin_label = pmr_dict[rtn_indx.get(i).unwrap()].log_nam.clone() + ":" + pmr_dict[rtn_indx.get(i).unwrap()].chan_nam.as_str();
                        // let pin_label = pmr_dict[rtn_indx.get(i).unwrap()].log_nam.clone();
                        let context = if i < rtn_indx.len() && pmr_dict.contains_key(rtn_indx.get(i).unwrap()) { "pin: ".to_string() + pin_label.as_str() } else { "".into() };

                        csv_file.write(format!("\"{}\",\"{}.{}\",\"{}\",\"=\"\"{}\"\"\",\"=\"\"{}\"\"\",\"{}\",\"{}\",\"{}\"", part_id, rec.test_num, i, rec.site_num, test_txt, context, lo_limit, result, hi_limit).as_bytes()).expect("Error while trying to write to the csv file.");
                        for id in &dtr_id_in_col_idx_order {
                            csv_file.write(format!(",\"{}\"", dtr_map.get(id).unwrap().text).as_bytes()).expect("Error while trying to write to the csv file.");
                        }
                        csv_file.write(b"\n").expect("Error while trying to write to the csv file.");
                    }
                    
                },

                // Part Results Record for summary files and popping part id
                StdfRecord::PRR(rec) => {
                    /////////////////////////////////////////////////////////
                    // Update parts summary
                    /////////////////////////////////////////////////////////
                    csv_part_summary_file.write(format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n", rec.part_id, rec.site_num, rec.test_t, rec.hard_bin, rec.soft_bin, rec.num_test, rec.x_coord, rec.y_coord).as_bytes())
                                                .expect("Error while trying to write to the part summary csv file.");
                    
                    // add to summary stats for stdf
                    let sum = stdf_summary_statistics.entry(rec.hard_bin).or_insert(BTreeMap::new());
                    let sum = sum.entry(rec.soft_bin).or_insert(BTreeMap::new());
                    if let Some(sum) = sum.get_mut(&rec.site_num) {
                        sum.test_count.push(rec.num_test);
                        sum.test_time.push(rec.test_t);
                    } else {
                        sum.insert(rec.site_num.to_owned(), StdfInfo {hard_bin: rec.hard_bin, site_number: rec.site_num, soft_bin: rec.soft_bin, test_count: vec![rec.num_test], test_time: vec![rec.test_t]});
                    }

                    /////////////////////////////////////////////////////////
                    // We've now encountered this Part ID, drop it from the list
                    /////////////////////////////////////////////////////////
                    first_pass_info.part_ids.remove(0);

                    /////////////////////////////////////////////////////////
                    // clear dtr info if set
                    /////////////////////////////////////////////////////////
                    for (_, info) in &mut dtr_map {
                        if info.clear_on_prr {
                            info.text = "".into();
                        }
                    }
                },

                StdfRecord::PMR(rec) => {
                    pmr_dict.insert(rec.pmr_indx, rec.clone());
                },

                _rec => {
                    // do nothing, just here temporarily
                }
            }
        }
    }

    /////////////////////////////////////////////////////////
    // Write out the STDF Summary
    /////////////////////////////////////////////////////////
    // write the header
    csv_stdf_summary_file
        .write(b"\"Hard Bin\",\" Soft Bin\",\" SiteNum\",\" Part Count\",\" Min Test Time\",\" Average Test Time\",\" Max Test Time\",\"Min Test Count\",\"Median Test Count\",\" Max Test Count\"\n")
        .expect("Error while trying to write to the stdf summary csv file.");

    // go through stats and write them out
    for (_, val) in stdf_summary_statistics {
        for (_, val) in val {
            for (_, val) in val {
                // grab appropriate stats to write
                csv_stdf_summary_file
                    .write(format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n", 
                                        val.hard_bin, val.soft_bin, val.site_number, val.test_time.len(), 
                                        vec_min(&val.test_time), vec_mean(&val.test_time), vec_max(&val.test_time),
                                        vec_min(&val.test_count), vec_median(&val.test_count), vec_max(&val.test_count)).as_bytes())
                    .expect("Error while trying to write to the stdf summary csv file.");
            }
        }
    }

    Ok(())
}


// this needs to be shifted into a structure to be easier to manage.
// pub fn init_stdf(stdf_path: &String, config_fname: &Option<String>) -> Result<(StdfReader<BufReader<File>>, Vec<DtrConfiguration>, FirstPassInfo), String> {
//     let dtr_config = load_dtr_config(config_fname);
//     let first_pass_info = first_pass_stdf(stdf_path, &dtr_config).unwrap_or(FirstPassInfo { min_site_num: 0, part_ids: Vec::new(), dtr_info: Vec::new() });
//     // open stdf file and start reading
//     let stdf_reader = match StdfReader::new(&stdf_path) {
//         // return if successful
//         Ok(stdf_reader) => stdf_reader,

//         // print full error and return the error message if not
//         Err(err) => {
//             println!("Error while loading stdf: {:?}\n", err);
//             return Err(err.msg);
//         }
//     };

//     Ok((stdf_reader, dtr_config, first_pass_info))
// }

// pub fn next_record(stdf_reader: StdfReader<BufReader<File>>, dtr_config: Vec<DtrConfiguration>, first_pass_info: FirstPassInfo) {
//     // grab next record

// }

// pub fn read_stdf(stdf_path: &String, config_fname: &Option<String>) -> Result<Vec<StdfRecord>, String> {
//     let dtr_config = load_dtr_config(config_fname);
//     let records = Vec::new();

//     let data = first_pass_stdf(stdf_path, &dtr_config).unwrap_or(FirstPassInfo { min_site_num: 0, part_ids: Vec::new(), dtr_info: Vec::new() });

//     Ok(records)
// }

pub fn convert_stdf2text(stdf_path: &String, txt_path: &String, pretty_print: bool, use_test_defaults: bool) -> Result<(), String> {
    // open csv files or error out
    let mut txt_file = std::fs::File::create(&txt_path).expect(format!("Error while trying to create the text file {}.", &txt_path).as_str());
    let mut i = 0;

    let mut test_defaults_ptr = HashMap::<u32, PTR>::new();
    let mut test_defaults_mpr = HashMap::<u32, MPR>::new();
    let mut test_defaults_ftr = HashMap::<u32, FTR>::new();

    // open stdf file and start reading
    let mut reader = match StdfReader::new(&stdf_path) {
        // return if successful
        Ok(reader) => reader,

        // print full error and return the error message if not
        Err(err) => {
            println!("Error while loading stdf: {:?}\n", err);
            return Err(err.msg);
        }
    };

    for stdf_rec in reader.get_record_iter() {
        if let Ok(stdf_rec) = stdf_rec {
            i = i + 1;

            let stdf_rec = if use_test_defaults {
                match &stdf_rec {
                    StdfRecord::PTR(rec) => {
                        StdfRecord::PTR(handle_ptr_defaults(rec, &mut test_defaults_ptr))
                    },
                    StdfRecord::MPR(rec) => {
                        StdfRecord::MPR(handle_mpr_defaults(rec, &mut test_defaults_mpr))
                    },
                    StdfRecord::FTR(rec) => {
                        StdfRecord::FTR(handle_ftr_defaults(rec, &mut test_defaults_ftr))
                    },
                    _ => stdf_rec,
                }
            } else {
                stdf_rec
            };
            
            let txt = rec_to_string::rec_to_string(&stdf_rec, pretty_print);
            let txt = if txt.is_empty() { format!("UNFORMATTED {:?}", &stdf_rec) } else { txt };
            
            writeln!(&mut txt_file, "{}", txt).expect("Error while trying to write to text file");
        }
    }

    Ok(())
}

pub fn convert_stdf2sqlite(stdf_path: &String, sqlite_path: &String, dtr_cfg_file: &Option<String>) -> Result<(), String> {
    Ok(())
}
