use std::fmt;
use rust_stdf::*;

pub fn rec_to_string(rec: &StdfRecord, pretty_print: bool) -> String {
    
    let (sep, indent) = if pretty_print { ("\n", "  ") } else { (" ", "") };
    let formatted_rec = match rec {
        // STDF v4 Records
        StdfRecord::FAR(rec) => {
            format!("FAR {{{}{}{}: {}{}{}{}: {}{}}}", 
                        sep, indent, "cpu_type", rec.cpu_type,
                        sep, indent, "stdf_ver", rec.stdf_ver,
                        sep)
        },
        StdfRecord::ATR(rec) => {
            format!("ATR {{{}{}{}: {}{}{}{}: {}{}}}", 
                        sep, indent, "mod_tim ", rec.mod_tim,
                        sep, indent, "cmd_line", rec.cmd_line,
                        sep)
        },
        StdfRecord::MIR(rec) => {
            format!("MIR {{{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}\
                            {}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}\
                            {}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}\
                            {}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}\
                            {}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}\
                            {}{}{}: {}{}{}{}: {}{}{}{}: {}{}\
                         }}", 
                    sep, indent, "setup_t ", rec.setup_t,
                    sep, indent, "start_t ", rec.start_t,
                    sep, indent, "stat_num", rec.stat_num,
                    sep, indent, "mode_cod", rec.mode_cod,
                    sep, indent, "rtst_cod", rec.rtst_cod,
                    sep, indent, "prot_cod", rec.prot_cod,
                    sep, indent, "burn_tim", rec.burn_tim,
                    sep, indent, "cmod_cod", rec.cmod_cod,
                    sep, indent, "lot_id  ", rec.lot_id,
                    sep, indent, "part_typ", rec.part_typ,
                    sep, indent, "node_nam", rec.node_nam,
                    sep, indent, "tstr_typ", rec.tstr_typ,
                    sep, indent, "job_nam ", rec.job_nam,
                    sep, indent, "job_rev ", rec.job_rev,
                    sep, indent, "sblot_id", rec.sblot_id,
                    sep, indent, "oper_nam", rec.oper_nam,
                    sep, indent, "exec_typ", rec.exec_typ,
                    sep, indent, "exec_ver", rec.exec_ver,
                    sep, indent, "test_cod", rec.test_cod,
                    sep, indent, "tst_temp", rec.tst_temp,
                    sep, indent, "user_txt", rec.user_txt,
                    sep, indent, "aux_file", rec.aux_file,
                    sep, indent, "pkg_typ ", rec.pkg_typ,
                    sep, indent, "famly_id", rec.famly_id,
                    sep, indent, "date_cod", rec.date_cod,
                    sep, indent, "facil_id", rec.facil_id,
                    sep, indent, "floor_id", rec.floor_id,
                    sep, indent, "proc_id ", rec.proc_id,
                    sep, indent, "oper_frq", rec.oper_frq,
                    sep, indent, "spec_nam", rec.spec_nam,
                    sep, indent, "spec_ver", rec.spec_ver,
                    sep, indent, "flow_id ", rec.flow_id,
                    sep, indent, "setup_id", rec.setup_id,
                    sep, indent, "dsgn_rev", rec.dsgn_rev,
                    sep, indent, "eng_id  ", rec.eng_id,
                    sep, indent, "rom_cod ", rec.rom_cod,
                    sep, indent, "serl_num", rec.serl_num,
                    sep, indent, "supr_nam", rec.supr_nam,
                    sep)
        },
        StdfRecord::MRR(rec) => {
            format!("MRR {{{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}}}", 
                        sep, indent, "finish_t", rec.finish_t,
                        sep, indent, "disp_cod", rec.disp_cod,
                        sep, indent, "usr_desc", rec.usr_desc,
                        sep, indent, "exc_desc", rec.exc_desc,
                        sep)
        },
        StdfRecord::PCR(rec) => {
            format!("PCR {{{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}}}", 
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "part_cnt", rec.part_cnt,
                        sep, indent, "rtst_cnt", rec.rtst_cnt,
                        sep, indent, "abrt_cnt", rec.abrt_cnt,
                        sep, indent, "good_cnt", rec.good_cnt,
                        sep, indent, "func_cnt", rec.func_cnt,
                        sep)
        },
        StdfRecord::HBR(rec) => {
            format!("HBR {{{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}}}", 
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "hbin_num", rec.hbin_num,
                        sep, indent, "hbin_cnt", rec.hbin_cnt,
                        sep, indent, "hbin_pf ", rec.hbin_pf,
                        sep, indent, "hbin_nam", rec.hbin_nam,
                        sep)
        },
        StdfRecord::SBR(rec) => {
            format!("SBR {{{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}}}", 
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "sbin_num", rec.sbin_num,
                        sep, indent, "sbin_cnt", rec.sbin_cnt,
                        sep, indent, "sbin_pf ", rec.sbin_pf,
                        sep, indent, "sbin_nam", rec.sbin_nam,
                        sep)
        },
        StdfRecord::PMR(rec) => {
            format!("PMR {{{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}}}", 
                        sep, indent, "pmr_indx", rec.pmr_indx,
                        sep, indent, "chan_typ", rec.chan_typ,
                        sep, indent, "chan_nam", rec.chan_nam,
                        sep, indent, "phy_nam ", rec.phy_nam,
                        sep, indent, "log_nam ", rec.log_nam,
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep)
        },
        StdfRecord::PGR(rec) => {
            format!("PGR {{{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {:?}{}}}", 
                        sep, indent, "grp_indx", rec.grp_indx,
                        sep, indent, "grp_nam ", rec.grp_nam,
                        sep, indent, "indx_cnt", rec.indx_cnt,
                        sep, indent, "pmr_indx", rec.pmr_indx,
                        sep)
        },
        StdfRecord::PLR(rec) => {
            format!("PLR {{{}{}{}: {}{}{}{}: {:?}{}{}{}: {:?}{}{}{}: {:?}{}{}{}: {:?}{}{}{}: {:?}{}{}{}: {:?}{}{}{}: {:?}{}}}", 
                        sep, indent, "grp_cnt ", rec.grp_cnt,
                        sep, indent, "grp_indx", rec.grp_indx,
                        sep, indent, "grp_mode", rec.grp_mode,
                        sep, indent, "grp_radx", rec.grp_radx,
                        sep, indent, "pgm_char", rec.pgm_char,
                        sep, indent, "rtn_char", rec.rtn_char,
                        sep, indent, "pgm_chal", rec.pgm_chal,
                        sep, indent, "rtn_chal", rec.rtn_chal,
                        sep)
        },
        StdfRecord::RDR(rec) => {
            format!("RDR {{{}{}{}: {}{}{}{}: {:?}{}}}", 
                        sep, indent, "grp_cnt", rec.num_bins,
                        sep, indent, "grp_indx", rec.rtst_bin,
                        sep)
        },
        StdfRecord::SDR(rec) => {
            format!("SDR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                        {}}}", 
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_grp", rec.site_grp,
                        sep, indent, "site_cnt", rec.site_cnt,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "hand_typ", rec.hand_typ,
                        sep, indent, "hand_id ", rec.hand_id,
                        sep, indent, "card_typ", rec.card_typ,
                        sep, indent, "card_id ", rec.card_id,
                        sep, indent, "load_typ", rec.load_typ,
                        sep, indent, "load_id ", rec.load_id,
                        sep, indent, "dib_typ ", rec.dib_typ,
                        sep, indent, "dib_id  ", rec.dib_id,
                        sep, indent, "cabl_typ", rec.cabl_typ,
                        sep, indent, "cabl_id ", rec.cabl_id,
                        sep, indent, "cont_typ", rec.cont_typ,
                        sep, indent, "cont_id ", rec.cont_id,
                        sep, indent, "lasr_typ", rec.lasr_typ,
                        sep, indent, "lasr_id ", rec.lasr_id,
                        sep, indent, "extr_typ", rec.extr_typ,
                        sep, indent, "extr_id ", rec.extr_id,
                        sep)
        },
        StdfRecord::WIR(rec) => {
            format!("WIR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                        {}}}", 
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_grp", rec.site_grp,
                        sep, indent, "start_t ", rec.start_t,
                        sep, indent, "wafer_id", rec.wafer_id,
                        sep)
        },
        StdfRecord::WRR(rec) => {
            format!("WRR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                        {}}}", 
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_grp", rec.site_grp,
                        sep, indent, "finish_t", rec.finish_t,
                        sep, indent, "part_cnt", rec.part_cnt,
                        sep, indent, "rtst_cnt", rec.rtst_cnt,
                        sep, indent, "abrt_cnt", rec.abrt_cnt,
                        sep, indent, "good_cnt", rec.good_cnt,
                        sep, indent, "func_cnt", rec.func_cnt,
                        sep, indent, "wafer_id", rec.wafer_id,
                        sep, indent, "fabwf_id", rec.fabwf_id,
                        sep, indent, "frame_id", rec.frame_id,
                        sep, indent, "mask_id ", rec.mask_id,
                        sep, indent, "usr_desc", rec.usr_desc,
                        sep, indent, "exc_desc", rec.exc_desc,
                        sep)
        },
        StdfRecord::WCR(rec) => {
            format!("WCR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                        {}}}", 
                        sep, indent, "wafr_siz", rec.wafr_siz,
                        sep, indent, "die_ht  ", rec.die_ht,
                        sep, indent, "die_wid ", rec.die_wid,
                        sep, indent, "wf_units", rec.wf_units,
                        sep, indent, "wf_flat ", rec.wf_flat,
                        sep, indent, "center_x", rec.center_x,
                        sep, indent, "center_y", rec.center_y,
                        sep, indent, "pos_x   ", rec.pos_x,
                        sep, indent, "pos_y   ", rec.pos_y,
                        sep)
        },
        StdfRecord::PIR(rec) => {
            format!("PIR {{{}{}{}: {}\
                            {}{}{}: {}\
                        {}}}", 
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep)
        },
        StdfRecord::PRR(rec) => {
            format!("PRR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                        {}}}", 
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "part_flg", rec.part_flg[0],
                        sep, indent, "num_test", rec.num_test,
                        sep, indent, "hard_bin", rec.hard_bin,
                        sep, indent, "soft_bin", rec.soft_bin,
                        sep, indent, "x_coord ", rec.x_coord,
                        sep, indent, "y_coord ", rec.y_coord,
                        sep, indent, "test_t  ", rec.test_t,
                        sep, indent, "part_id ", rec.part_id,
                        sep, indent, "part_txt", rec.part_txt,
                        sep, indent, "part_fix", rec.part_fix,
                        sep)
        },
        StdfRecord::TSR(rec) => {
            format!("TSR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                        {}}}", 
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "test_typ", rec.test_typ,
                        sep, indent, "test_num", rec.test_num,
                        sep, indent, "exec_cnt", rec.exec_cnt,
                        sep, indent, "fail_cnt", rec.fail_cnt,
                        sep, indent, "alrm_cnt", rec.alrm_cnt,
                        sep, indent, "test_nam", rec.test_nam,
                        sep, indent, "seq_name", rec.seq_name,
                        sep, indent, "test_lbl", rec.test_lbl,
                        sep, indent, "opt_flag", rec.opt_flag[0],
                        sep, indent, "test_tim", rec.test_tim,
                        sep, indent, "test_min", rec.test_min,
                        sep, indent, "test_max", rec.test_max,
                        sep, indent, "tst_sums", rec.tst_sums,
                        sep, indent, "tst_sqrs", rec.tst_sqrs,
                        sep)
        },
        StdfRecord::PTR(rec) => {
            format!("PTR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                        {}}}", 
                        sep, indent, "test_num", rec.test_num,
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "test_flg", rec.test_flg[0],
                        sep, indent, "parm_flg", rec.parm_flg[0],
                        sep, indent, "result  ", rec.result  ,
                        sep, indent, "test_txt", rec.test_txt,
                        sep, indent, "alarm_id", rec.alarm_id,
                        sep, indent, "opt_flag", if let Some(v) = &rec.opt_flag { format!("{}", v[0]) } else { "None".into() },
                        sep, indent, "res_scal", if let Some(v) = &rec.res_scal { format!("{}", v) } else { "None".into() },
                        sep, indent, "llm_scal", if let Some(v) = &rec.llm_scal { format!("{}", v) } else { "None".into() },
                        sep, indent, "hlm_scal", if let Some(v) = &rec.hlm_scal { format!("{}", v) } else { "None".into() },
                        sep, indent, "lo_limit", if let Some(v) = &rec.lo_limit { format!("{}", v) } else { "None".into() },
                        sep, indent, "hi_limit", if let Some(v) = &rec.hi_limit { format!("{}", v) } else { "None".into() },
                        sep, indent, "units   ", if let Some(v) = &rec.units    { format!("{}", v) } else { "None".into() },
                        sep, indent, "c_resfmt", if let Some(v) = &rec.c_resfmt { format!("{}", v) } else { "None".into() },
                        sep, indent, "c_llmfmt", if let Some(v) = &rec.c_llmfmt { format!("{}", v) } else { "None".into() },
                        sep, indent, "c_hlmfmt", if let Some(v) = &rec.c_hlmfmt { format!("{}", v) } else { "None".into() },
                        sep, indent, "lo_spec ", if let Some(v) = &rec.lo_spec  { format!("{}", v) } else { "None".into() },
                        sep, indent, "hi_spec ", if let Some(v) = &rec.hi_spec  { format!("{}", v) } else { "None".into() },
                        sep)
        },
        StdfRecord::MPR(rec) => {
            format!("MPR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                            {}{}{}: {:?}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                        {}}}", 
                        sep, indent, "test_num", rec.test_num,
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "test_flg", rec.test_flg[0],
                        sep, indent, "parm_flg", rec.parm_flg[0],
                        sep, indent, "rtn_icnt", rec.rtn_icnt,
                        sep, indent, "rslt_cnt", rec.rslt_cnt,
                        sep, indent, "rtn_stat", rec.rtn_stat,
                        sep, indent, "rtn_rslt", rec.rtn_rslt,
                        sep, indent, "test_txt", rec.test_txt,
                        sep, indent, "alarm_id", rec.alarm_id,
                        sep, indent, "opt_flag", if let Some(v) = &rec.opt_flag { format!("{}", v[0]) } else { "None".into() },
                        sep, indent, "res_scal", if let Some(v) = &rec.res_scal { format!("{}", v) } else { "None".into() },
                        sep, indent, "llm_scal", if let Some(v) = &rec.llm_scal { format!("{}", v) } else { "None".into() },
                        sep, indent, "hlm_scal", if let Some(v) = &rec.hlm_scal { format!("{}", v) } else { "None".into() },
                        sep, indent, "lo_limit", if let Some(v) = &rec.lo_limit { format!("{}", v) } else { "None".into() },
                        sep, indent, "hi_limit", if let Some(v) = &rec.hi_limit { format!("{}", v) } else { "None".into() },
                        sep, indent, "start_in", if let Some(v) = &rec.start_in { format!("{}", v) } else { "None".into() },
                        sep, indent, "incr_in ", if let Some(v) = &rec.incr_in  { format!("{}", v) } else { "None".into() },
                        sep, indent, "rtn_indx", if let Some(v) = &rec.rtn_indx { format!("{:?}", v) } else { "None".into() },
                        sep, indent, "units   ", if let Some(v) = &rec.units    { format!("{}", v) } else { "None".into() },
                        sep, indent, "units_in", if let Some(v) = &rec.units_in { format!("{}", v) } else { "None".into() },
                        sep, indent, "c_resfmt", if let Some(v) = &rec.c_resfmt { format!("{}", v) } else { "None".into() },
                        sep, indent, "c_llmfmt", if let Some(v) = &rec.c_llmfmt { format!("{}", v) } else { "None".into() },
                        sep, indent, "c_hlmfmt", if let Some(v) = &rec.c_hlmfmt { format!("{}", v) } else { "None".into() },
                        sep, indent, "lo_spec ", if let Some(v) = &rec.lo_spec  { format!("{}", v) } else { "None".into() },
                        sep, indent, "hi_spec ", if let Some(v) = &rec.hi_spec  { format!("{}", v) } else { "None".into() },
                        sep)
        },
        StdfRecord::FTR(rec) => {
            format!("FTR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                            {}{}{}: {:?}\
                            {}{}{}: {:?}\
                            {}{}{}: {:?}\
                            {}{}{}: {:?}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                        {}}}", 
                        sep, indent, "test_num", rec.test_num,
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "test_flg", rec.test_flg[0],
                        sep, indent, "cycl_cnt", rec.cycl_cnt,
                        sep, indent, "rel_vadr", rec.rel_vadr,
                        sep, indent, "rept_cnt", rec.rept_cnt,
                        sep, indent, "num_fail", rec.num_fail,
                        sep, indent, "xfail_ad", rec.xfail_ad,
                        sep, indent, "yfail_ad", rec.yfail_ad,
                        sep, indent, "vect_off", rec.vect_off,
                        sep, indent, "rtn_icnt", rec.rtn_icnt,
                        sep, indent, "pgm_icnt", rec.pgm_icnt,
                        sep, indent, "rtn_indx", rec.rtn_indx,
                        sep, indent, "rtn_stat", rec.rtn_stat,
                        sep, indent, "pgm_indx", rec.pgm_indx,
                        sep, indent, "pgm_stat", rec.pgm_stat,
                        sep, indent, "fail_pin", rec.fail_pin,
                        sep, indent, "vect_nam", rec.vect_nam,
                        sep, indent, "time_set", rec.time_set,
                        sep, indent, "op_code ", rec.op_code,
                        sep, indent, "test_txt", rec.test_txt,
                        sep, indent, "alarm_id", rec.alarm_id,
                        sep, indent, "prog_txt", rec.prog_txt,
                        sep, indent, "rslt_txt", rec.rslt_txt,
                        sep, indent, "patg_num", rec.patg_num,
                        sep, indent, "spin_map", rec.spin_map,
                        sep)
        },
        StdfRecord::BPS(rec) => {
            format!("BPS {{{}{}{}: {}\
                        {}}}", 
                        sep, indent, "seq_name", rec.seq_name,
                        sep)
        },
        StdfRecord::EPS(_) => {
            format!("EPS {{\
                        {}}}",
                        sep)
        },
        StdfRecord::GDR(rec) => {
            let mut ret_string = format!("GDR {{{}{}{}: {}", sep, indent, "fld_cnt", rec.fld_cnt);
            for (i, val) in rec.gen_data.iter().enumerate() {
                ret_string = ret_string + format!("{}{}{}[{}]: {:?}", sep, indent, "gen_data", i, val).as_str();
            }
            ret_string + format!("{}}}", sep).as_str()
        },
        StdfRecord::DTR(rec) => {
            format!("DTR {{{}{}{}: {}\
                        {}}}", 
                        sep, indent, "text_dat", rec.text_dat,
                        sep)
        },

        // STDF V4-2007 Records
        StdfRecord::VUR(rec) => {
            format!("VUR {{{}{}{}: {}\
                        {}}}", 
                        sep, indent, "upd_nam", rec.upd_nam,
                        sep)
        },
        StdfRecord::PSR(rec) => {
            format!("PSR {{{}{}{}: {}\
                        {}{}{}: {}\
                        {}{}{}: {}\
                        {}{}{}: {}\
                        {}{}{}: {}\
                        {}{}{}: {}\
                        {}{}{}: {:?}\
                        {}{}{}: {:?}\
                        {}{}{}: {:?}\
                        {}{}{}: {:?}\
                        {}{}{}: {:?}\
                        {}{}{}: {:?}\
                        {}{}{}: {:?}\
                        {}}}", 
                        sep, indent, "cont_flg", rec.cont_flg[0],
                        sep, indent, "psr_indx", rec.psr_indx,
                        sep, indent, "psr_nam,", rec.psr_nam,
                        sep, indent, "opt_flg ", rec.opt_flg[0],
                        sep, indent, "totp_cnt", rec.totp_cnt,
                        sep, indent, "locp_cnt", rec.locp_cnt,
                        sep, indent, "pat_bgn ", rec.pat_bgn ,
                        sep, indent, "pat_end ", rec.pat_end ,
                        sep, indent, "pat_file", rec.pat_file,
                        sep, indent, "pat_lbl ", rec.pat_lbl ,
                        sep, indent, "file_uid", rec.file_uid,
                        sep, indent, "atpg_dsc", rec.atpg_dsc,
                        sep, indent, "src_id  ", rec.src_id  ,
                        sep)
        },
        StdfRecord::STR(rec) => {
            format!("STR {{{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}\
                            {}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {:?}{}{}{}: {:?}{}{}{}: {}\
                            {}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}\
                            {}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}{}{}{}: {}\
                            {}{}{}: {:?}{}{}{}: {:?}{}{}{}: {:?}{}{}{}: {}{}{}{}: {:?}{}{}{}: {}{}{}{}: {:?}{}{}{}: {}\
                            {}{}{}: {:?}{}{}{}: {}{}{}{}: {:?}{}{}{}: {}{}{}{}: {:?}{}{}{}: {}{}{}{}: {:?}{}{}{}: {}\
                            {}{}{}: {:?}{}{}{}: {}{}{}{}: {:?}{}{}{}: {}{}{}{}: {:?}{}{}{}: {}{}{}{}: {:?}{}{}{}: {}\
                            {}{}{}: {:?}{}{}{}: {}{}{}{}: {:?}\
                        {}}}",
                        sep, indent, "cont_flg", rec.cont_flg[0],
                        sep, indent, "test_num", rec.test_num,
                        sep, indent, "head_num", rec.head_num,
                        sep, indent, "site_num", rec.site_num,
                        sep, indent, "psr_ref ", rec.psr_ref,
                        sep, indent, "test_flg", rec.test_flg[0],
                        sep, indent, "log_typ ", rec.log_typ,
                        sep, indent, "test_txt", rec.test_txt,
                        sep, indent, "alarm_id", rec.alarm_id,
                        sep, indent, "prog_txt", rec.prog_txt,
                        sep, indent, "rslt_txt", rec.rslt_txt,
                        sep, indent, "z_val   ", rec.z_val,
                        sep, indent, "fmu_flg ", rec.fmu_flg[0],
                        sep, indent, "mask_map", rec.mask_map,
                        sep, indent, "fal_map ", rec.fal_map,
                        sep, indent, "cyc_cnt ", rec.cyc_cnt,
                        sep, indent, "totf_cnt", rec.totf_cnt,
                        sep, indent, "totl_cnt", rec.totl_cnt,
                        sep, indent, "cyc_base", rec.cyc_base,
                        sep, indent, "bit_base", rec.bit_base,
                        sep, indent, "cond_cnt", rec.cond_cnt,
                        sep, indent, "lim_cnt ", rec.lim_cnt,
                        sep, indent, "cyc_size", rec.cyc_size,
                        sep, indent, "pmr_size", rec.pmr_size,
                        sep, indent, "chn_size", rec.chn_size,
                        sep, indent, "pat_size", rec.pat_size,
                        sep, indent, "bit_size", rec.bit_size,
                        sep, indent, "u1_size ", rec.u1_size,
                        sep, indent, "u2_size ", rec.u2_size,
                        sep, indent, "u3_size ", rec.u3_size,
                        sep, indent, "utx_size", rec.utx_size,
                        sep, indent, "cap_bgn ", rec.cap_bgn,
                        sep, indent, "lim_indx", rec.lim_indx,
                        sep, indent, "lim_spec", rec.lim_spec,
                        sep, indent, "cond_lst", rec.cond_lst,
                        sep, indent, "cyc_cnt ", rec.cyc_cnt,
                        sep, indent, "cyc_ofst", rec.cyc_ofst,
                        sep, indent, "pmr_cnt ", rec.pmr_cnt,
                        sep, indent, "pmr_indx", rec.pmr_indx,
                        sep, indent, "chn_cnt ", rec.chn_cnt,
                        sep, indent, "chn_num ", rec.chn_num,
                        sep, indent, "exp_cnt ", rec.exp_cnt,
                        sep, indent, "exp_data", rec.exp_data,
                        sep, indent, "cap_cnt ", rec.cap_cnt,
                        sep, indent, "cap_data", rec.cap_data,
                        sep, indent, "new_cnt ", rec.new_cnt,
                        sep, indent, "new_data", rec.new_data,
                        sep, indent, "pat_cnt ", rec.pat_cnt,
                        sep, indent, "pat_num ", rec.pat_num,
                        sep, indent, "bpos_cnt", rec.bpos_cnt,
                        sep, indent, "bit_pos ", rec.bit_pos,
                        sep, indent, "usr1_cnt", rec.usr1_cnt,
                        sep, indent, "usr1    ", rec.usr1,
                        sep, indent, "usr2_cnt", rec.usr2_cnt,
                        sep, indent, "usr2    ", rec.usr2,
                        sep, indent, "usr3_cnt", rec.usr3_cnt,
                        sep, indent, "usr3    ", rec.usr3,
                        sep, indent, "txt_cnt ", rec.txt_cnt,
                        sep, indent, "user_txt", rec.user_txt,
                        sep)
        },
        StdfRecord::NMR(rec) => {
            format!("NMR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                            {}{}{}: {:?}\
                        {}}}", 
                        sep, indent, "cont_flg", rec.cont_flg[0],
                        sep, indent, "totm_cnt", rec.totm_cnt,
                        sep, indent, "locm_cnt", rec.locm_cnt,
                        sep, indent, "pmr_indx", rec.pmr_indx,
                        sep, indent, "atpg_nam", rec.atpg_nam,
                        sep)
        },
        StdfRecord::CNR(rec) => {
            format!("CNR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                        {}}}", 
                        sep, indent, "chn_num ", rec.chn_num,
                        sep, indent, "bit_pos ", rec.bit_pos,
                        sep, indent, "cell_nam", rec.cell_nam,
                        sep)
        },
        StdfRecord::SSR(rec) => {
            format!("SSR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                        {}}}", 
                        sep, indent, "ssr_nam ", rec.ssr_nam,
                        sep, indent, "chn_cnt ", rec.chn_cnt,
                        sep, indent, "chn_list", rec.chn_list,
                        sep)
        },
        StdfRecord::CDR(rec) => {
            format!("CDR {{{}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                            {}{}{}: {}\
                            {}{}{}: {}\
                            {}{}{}: {:?}\
                        {}}}", 
                        sep, indent, "cont_flg", rec.cont_flg[0],
                        sep, indent, "cdr_indx", rec.cdr_indx,
                        sep, indent, "chn_nam ", rec.chn_nam ,
                        sep, indent, "chn_len,", rec.chn_len,
                        sep, indent, "sin_pin ", rec.sin_pin ,
                        sep, indent, "sout_pin", rec.sout_pin,
                        sep, indent, "mstr_cnt", rec.mstr_cnt,
                        sep, indent, "m_clks  ", rec.m_clks  ,
                        sep, indent, "slav_cnt", rec.slav_cnt,
                        sep, indent, "s_clks  ", rec.s_clks  ,
                        sep, indent, "inv_val ", rec.inv_val ,
                        sep, indent, "lst_cnt ", rec.lst_cnt ,
                        sep, indent, "cell_lst", rec.cell_lst,
                        sep)
        },

        // Unhandled
        rec => {format!("UNHANDLED: {:?}", &rec)}
    };

    formatted_rec.to_owned()
}
