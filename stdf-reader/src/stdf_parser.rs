use std::{any::Any, collections::HashMap, fs::File, io::BufReader};

use regex::Regex;
use rust_stdf::stdf_file::RecordIter;
pub use rust_stdf::{stdf_file::{self, StdfReader}, *};

// #[macro_use]
// extern crate ini;

#[derive(Debug, Clone)]
pub struct DtrConfiguration {
    name: String,
    regex: Regex,
    id_fmt: String,
    attach_to_records: Vec<String>,
    text_fmt: String,
    clear_on_prr: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DtrInfo {
    pub id: String,
    pub attach_to: Vec<String>,
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

type TestDefaultsFtr = HashMap<u32, FTR>;
type TestDefaultsMpr = HashMap<u32, MPR>;
type TestDefaultsPtr = HashMap<u32, PTR>;

pub struct StdfParser {
    reader: stdf_file::StdfReader<BufReader<File>>,
    dtr_config: Vec<DtrConfiguration>,
    dtr_info: Vec<DtrInfo>,

    test_defaults_ftr: TestDefaultsFtr,
    test_defaults_mpr: TestDefaultsMpr,
    test_defaults_ptr: TestDefaultsPtr,
}

impl StdfParser {
    pub fn new(path: &String, config_fname: &Option<String>) -> Result<Self, String> {
        let dtr_config = Self::load_dtr_config(config_fname);
        let reader = StdfReader::new(path).map_err(|e| e.to_string())?;

        Ok(Self { 
            reader,
            dtr_config,
            dtr_info: Vec::new(),
            test_defaults_ftr: TestDefaultsFtr::new(),
            test_defaults_mpr: TestDefaultsMpr::new(),
            test_defaults_ptr: TestDefaultsPtr::new(),
        })
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
                    attach_to_records: val["link_to_records"].to_owned().unwrap_or("".to_string()).split(",").into_iter().map(|s| s.to_ascii_uppercase().into()).collect(),
                    text_fmt: val["text_fmt"].to_owned().unwrap_or("".to_string()),
                    clear_on_prr: val["clear_on_prr"].to_owned().unwrap_or("False".into()).eq_ignore_ascii_case("true")
                })
            }
        }
        ret_val
    }

    fn parse_dtr(&mut self, rec: &DTR) {
        // Look through DTR configuration passed in and find matches to regex
        for val in &self.dtr_config {
    
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

                self.dtr_info.push(DtrInfo {
                    id: id.to_owned(),
                    attach_to: val.attach_to_records.to_owned(),
                    text: text.to_owned(),
                    clear_on_prr: val.clear_on_prr,
                });
            }
        }
    }

    fn handle_mpr_defaults(&mut self, rec: &MPR) -> MPR {
        // clone the MPR record for manupulation
        let mut rec = rec.clone();
    
        if rec.opt_flag != None && rec.opt_flag.unwrap()[0] & 0x40 != 0 { rec.lo_limit = None; rec.llm_scal = None; }
        if rec.opt_flag != None && rec.opt_flag.unwrap()[0] & 0x80 != 0 { rec.hi_limit = None; rec.hlm_scal = None; }
        
        if let Some(defaults) = self.test_defaults_mpr.get(&rec.test_num) {
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
            self.test_defaults_mpr.insert(rec.test_num, rec.clone());
        }
    
        rec
    }
    
    fn handle_ptr_defaults(&mut self, rec: &PTR) -> PTR {
        // clone the PTR record for manupulation
        let mut rec = rec.clone();
    
        if rec.opt_flag != None && rec.opt_flag.unwrap()[0] & 0x40 != 0 { rec.lo_limit = None; rec.llm_scal = None; }
        if rec.opt_flag != None && rec.opt_flag.unwrap()[0] & 0x80 != 0 { rec.hi_limit = None; rec.hlm_scal = None; }
    
        if let Some(defaults) = self.test_defaults_ptr.get(&rec.test_num) {
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
            self.test_defaults_ptr.insert(rec.test_num, rec.clone());
        }
    
        rec
    }
    
    fn handle_ftr_defaults(&mut self, rec: &FTR) -> FTR {
        // clone the PTR record for manupulation
        let mut rec = rec.clone();
    
        if let Some(defaults) = self.test_defaults_ftr.get(&rec.test_num) {
            // We have access to the defaults, update what needs updating
            // Only the last 2 fields come over for defaults
            // if rec.opt_flag[0] & 0x01 > 0 { rec.cycl_cnt = defaults.cycl_cnt.to_owned(); }
            // if rec.opt_flag[0] & 0x02 > 0 { rec.rel_vadr = defaults.rel_vadr.to_owned(); }
            // if rec.opt_flag[0] & 0x04 > 0 { rec.rept_cnt = defaults.rept_cnt.to_owned(); }
            // if rec.opt_flag[0] & 0x08 > 0 { rec.num_fail = defaults.num_fail.to_owned(); }
            // if rec.opt_flag[0] & 0x10 > 0 { rec.xfail_ad = defaults.xfail_ad.to_owned(); }
            // if rec.opt_flag[0] & 0x10 > 0 { rec.yfail_ad = defaults.yfail_ad.to_owned(); }
            // if rec.opt_flag[0] & 0x20 > 0 { rec.vect_off = defaults.vect_off.to_owned(); }
            // if rec.rtn_icnt == 0          { rec.rtn_indx = defaults.rtn_indx.to_owned(); }
            // if rec.rtn_icnt == 0          { rec.rtn_stat = defaults.rtn_stat.to_owned(); }
            // if rec.pgm_icnt == 0          { rec.pgm_indx = defaults.pgm_indx.to_owned(); }
            // if rec.pgm_icnt == 0          { rec.pgm_stat = defaults.pgm_stat.to_owned(); }
            // if rec.fail_pin.len() == 0    { rec.fail_pin = defaults.fail_pin.to_owned(); }
            // if rec.vect_nam.len() == 0    { rec.vect_nam = defaults.vect_nam.to_owned(); }
            // if rec.time_set.len() == 0    { rec.time_set = defaults.time_set.to_owned(); }
            // if rec.op_code.len() == 0     { rec.op_code  = defaults.op_code.to_owned();  }
            // if rec.test_txt.len() == 0    { rec.test_txt = defaults.test_txt.to_owned(); }
            // if rec.alarm_id.len() == 0    { rec.alarm_id = defaults.alarm_id.to_owned(); }
            // if rec.prog_txt.len() == 0    { rec.prog_txt = defaults.prog_txt.to_owned(); }
            // if rec.rslt_txt.len() == 0    { rec.rslt_txt = defaults.rslt_txt.to_owned(); }
            if rec.patg_num == 255        { rec.patg_num = defaults.patg_num.to_owned(); }
            if rec.spin_map.len() == 0    { rec.spin_map = defaults.spin_map.to_owned(); }
        } else {
            // don't have the updates, store the record for later use
            self.test_defaults_ftr.insert(rec.test_num, rec.clone());
        }
    
        rec
    }

    pub fn get_all_recs(&mut self) -> Result<Vec<StdfRecord>, String> {
        let mut recs = Vec::<StdfRecord>::new();

        for rec in self.reader.get_record_iter() {
            if let Ok(rec) = rec {
                recs.push(rec);
            } else {
                break;
            }
        }

        if recs.len() == 0 {
            Err("No records found.".to_string())
        } else {
            Ok(recs)
        }
    }

    pub fn get_attached_dtr_info(&mut self, rec: &StdfRecord) -> Vec<DtrInfo> {
        let mut ret_val = Vec::<DtrInfo>::new();
        let typename = format!("{:?}", rec).split(" ").next().unwrap_or("").to_string();

        if typename == "" { return ret_val; }
    
        for dtr in &self.dtr_info {
            if dtr.attach_to.contains(&typename) {
                ret_val.push(dtr.to_owned());
            }
        }
    
        ret_val
    }

    pub fn next(&mut self) -> Option<Result<(StdfRecord, Vec<DtrInfo>), String>> {
        if let Some(stdf_rec) = self.reader.get_record_iter().next() {
            match stdf_rec {
                Ok(stdf_rec) => {
                    let mut ret_rec = stdf_rec.to_owned();
                    let attached_dtr_info = Vec::<DtrInfo>::new();

                    if let StdfRecord::DTR(rec) = &stdf_rec {
                        // handle DTR record
                        self.parse_dtr(&rec);
                    } else if let StdfRecord::MPR(rec) = &stdf_rec {
                        // handle MPR record defaults
                        ret_rec = StdfRecord::MPR(self.handle_mpr_defaults(&rec));
                    } else if let StdfRecord::PTR(rec) = &stdf_rec {
                        // handle PTR record defaults
                        ret_rec = StdfRecord::PTR(self.handle_ptr_defaults(&rec));
                    } else if let StdfRecord::FTR(rec) = &stdf_rec {
                        // handle FTR record defaults
                        ret_rec = StdfRecord::FTR(self.handle_ftr_defaults(&rec));
                    }

                    // get attached DTR info
                    // let attached_dtr_info = self.get_attached_dtr_info(&ret_rec);

                    Some(Ok((ret_rec, attached_dtr_info)))
                }
                Err(e) => Some(Err(e.to_string())),
            }
            // record
            // rec.map_err(|e| e.to_string())
        } else {
            None
        }
    }
}