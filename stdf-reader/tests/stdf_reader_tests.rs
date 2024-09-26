const DTR_CONFIG_FILE_EXAMPLE: &str = "
# Determines how to treat DTR statements encountered, if not in here they're dropped

[conditions]                    # label for dtr lookup
regex=COND: *(.*)=(.*)          # no default, required
id_fmt=$1                       # optional, used to determine overridable when linking to record
link_to_records=PTR,FTR,MPR     # records that need to reference it, leave blank if no dependence present
text_fmt=$2                     # actual text to store into record
clear_on_prr=true               # if linking to record, data is sticky unless clear_on_pir specified";

use std::io::Write;

use stdf_reader::*;

#[test]
fn parse_dtr() {
    // create fake DTR record
    let dtr = rust_stdf::DTR {
        text_dat: "COND: key=value".into(),
    };

    // create dtr configuration ini file
    std::fs::File::create("dtr_config.ini").unwrap().write_all(DTR_CONFIG_FILE_EXAMPLE.as_bytes()).unwrap();

    // load configuration file
    let dtr_cfg = stdf_reader::load_dtr_config(&Some("dtr_config.ini".into()));
    println!("dtr_cfg: {:#?}", dtr_cfg);

    // delete configuration file
    std::fs::remove_file("dtr_config.ini").unwrap();

    // run data
    let parsed_dtr = stdf_reader::parse_dtr(&dtr, &dtr_cfg);
    println!("parsed_dtr: {:#?}", parsed_dtr);

    // test results
    assert_eq!(parsed_dtr, Some(DtrInfo {uuid: "b692cf3c".into(), id: "key".into(), inject_into: vec!["PTR".into(), "FTR".into(), "MPR".into()], text: "value".into(), clear_on_prr: true}))
}

// #[test]
// fn convert_stdf2csv() {
//     // create dtr configuration ini file
//     std::fs::File::create("dtr_config.ini").unwrap().write_all(DTR_CONFIG_FILE_EXAMPLE.as_bytes()).unwrap();

//     // run data
//     stdf_reader::convert_stdf2csv(&"test.stdf.gz".into(), &"test.stdf.gz.csv".into(), &Some("dtr_config.ini".into()))
//         .expect("There was an error loading reference stdf.");

//     // delete configuration file
//     std::fs::remove_file("dtr_config.ini").unwrap();

// }

// #[test]
// fn convert_stdf2text() {
//     // run data
//     stdf_reader::convert_stdf2text(&"test.stdf.gz".to_string(), &"test.stdf.gz.txt".to_string(), false, false)
//         .expect("There was an error loading reference stdf.");
// }