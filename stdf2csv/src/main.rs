use std::io::stdout;

use stdf_reader::convert_stdf2csv;
use argparse::{ArgumentParser, Collect, Store};

fn main() {
    let mut stdf_filenames = Vec::<String>::new();
    let mut csv_filename = String::new();
    let mut dtr_cfg_filename = String::new();

    // force lifetime for Argument parser to be short
    {
        // Create ArgumentParser variable
        let mut ap = ArgumentParser::new();

        // Application description
        ap.set_description("Takes an STDF and converts it to a human readable text version");

        // Add all arguments and associated variables
        ap.refer(&mut stdf_filenames).add_argument("Stdf Input", Collect, "Stdf input file to be converted").required();
        // ap.refer(&mut csv_filename)
        //     .add_option(&["-o", "--output"],
        //                 Store,
        //                 "Override output file, if not used will default to [Stdf Input].csv");
        ap.refer(&mut dtr_cfg_filename)
            .add_option(&["-d", "--dtr-file"],
                        Store,
                        "Can be used to specify how to handle DTR's in the STDF, by default DTR's are ignored");

        // parse arguments and store
        ap.parse_args_or_exit();
    }

    if stdf_filenames.len() <= 0 {
        println!("No stdf files provided.");
        return;
    }

    let dtr_cfg_filename = if dtr_cfg_filename.is_empty() { None } else { Some(dtr_cfg_filename) };
    for stdf_filename in stdf_filenames {
        csv_filename = stdf_filename.clone() + ".csv";

        // do actual conversion
        convert_stdf2csv(&stdf_filename, &csv_filename, &dtr_cfg_filename).unwrap();
    }
}
