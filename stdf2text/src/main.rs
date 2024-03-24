use stdf_reader::convert_stdf2text;
use argparse::{ArgumentParser, Collect, StoreTrue};

fn main() {
    let mut pretty_print = false;
    let mut raw = false;
    let mut stdf_filenames = Vec::<String>::new();

    // force lifetime for Argument parser to be short
    {
        // Create ArgumentParser variable
        let mut ap = ArgumentParser::new();

        // Application description
        ap.set_description("Takes an STDF and converts it to a human readable text version");

        // Add all arguments and associated variables
        ap.refer(&mut stdf_filenames).add_argument("Stdf Input", Collect, "Stdf input file to be converted").required();
        // ap.refer(&mut text_filename)
        //     .add_option(&["-o", "--output"],
        //                 Store,
        //                 "Override output file, if not used will default to [Stdf Input].txt");
        ap.refer(&mut pretty_print)
            .add_option(&["-p", "--prettyprint"], 
                StoreTrue, 
                "Change output format to a prettier print version");
        ap.refer(&mut raw)
            .add_option(&["-r", "--raw"], 
                StoreTrue, 
                "No post processing will be done on the STDF, the exact input is output (PTR/MPR will not be restored with default lookups)");

        // parse arguments and store
        ap.parse_args_or_exit();
    }

    if stdf_filenames.len() <= 0 {
        println!("No stdf files provided.");
        return;
    }

    for stdf_filename in stdf_filenames {
        let text_filename = stdf_filename.clone() + ".txt";

        println!("Convert stdf file '{}' to text file '{}'", stdf_filename, text_filename);
        convert_stdf2text(&stdf_filename, &text_filename, pretty_print, !raw).unwrap();
    }
}
