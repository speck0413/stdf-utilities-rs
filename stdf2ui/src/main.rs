mod app;
mod ui;

use std::{collections::HashMap, error::Error, io::{stderr, Write}, sync::mpsc::{Receiver, Sender}, thread, time::Duration};

use app::{ActiveWidget, App};
use ui::ui;
// Import the necessary modules
use argparse::{ArgumentParser, Store, StoreOption};
use stdf_reader::*;

// use color_eyre::config::HookBuilder;
use crossterm::{
    event::{self, Event}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::prelude::*;

// Define a struct to hold the arguments
struct Arguments {
    stdf_filename: String,
    dtr_config_fname: Option<String>,
}

// structure used for sending messages between threads
struct WorkerMessage {
    _stdf_filename: String,
    log_entry: String,
    _result: f32,
}

// Function to parse the arguments
fn parse_arguments() -> Arguments {
    // Create a mutable vector to store the STDF filenames
    let mut args = Arguments { stdf_filename: String::new(), dtr_config_fname: None };

    // Create ArgumentParser variable
    let mut ap = ArgumentParser::new();

    // Set the application description
    ap.set_description("Takes an STDF and converts it to a human readable text version");

    // Add all arguments and associated variables
    // Here we are adding an argument for the STDF input file
    ap.refer(&mut args.stdf_filename)
    .add_argument("Stdf Input", Store, "Stdf input file to be converted").required();

    ap.refer(&mut args.dtr_config_fname)
    .add_argument("Dtr Config", StoreOption, "Optional DTR Configuration file to be used to determine how DTR's are handled/attached to other records. If not provided, DTR's will be ignored.");
    // ap.refer(&mut args.dtr_config_fname)
    //         .add_option(&["-c", "--config_fname"], 
    //             StoreOption, 
    //             "Dtr configuration file used to determine how DTR's are handled/attached to other records. If not provided, DTR's will be ignored.");

    // Parse the arguments and store them
    ap.parse_args_or_exit();

    // Drop the ArgumentParser so we can return the arguments
    std::mem::drop(ap);

    // Return the arguments in a struct
    args
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

fn rec_to_worker_message(stdf_filename: &String, site_to_part_idx: &HashMap<u8, u32>, rec: &StdfRecord) -> Option<WorkerMessage> {
    let fail_type_regex = regex::Regex::new(r"S[0-9]+_").unwrap();

    // do the thing
    match rec {
        // For all other record types, do nothing
        StdfRecord::DTR(rec) => {
            Some(WorkerMessage {
                _stdf_filename: stdf_filename.to_owned(),
                log_entry: format!("DTR: {}", rec.text_dat.to_owned()),
                _result: 0.0,
            })
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
            text = format!("{:04}  {}{}  {}{}{}{}{} {}", part_idx, text, rec.test_txt, llm, llm_cmp, result, hlm_cmp, hlm, units);

            let result = rec.result;
            Some(WorkerMessage {
                _stdf_filename: stdf_filename.to_owned(),
                log_entry: text,
                _result: result,
            })
        },
        StdfRecord::FTR(rec) => {
            // add to log
            let part_idx = site_to_part_idx.get(&rec.site_num).unwrap_or(&0);
            let opt_flag = rec.opt_flag[0];
            let test_flag = rec.test_flg[0];
            let mut text = "".to_string();
            let failed_string = if fail_type_regex.is_match(&rec.test_txt) { "Failed  " } else { "failed  " };

            let result = if opt_flag & 0x08 == 0 {
                // return num_fail
                if rec.num_fail > 0 {
                    text = failed_string.to_string();
                }
                rec.num_fail as f32
            } else {
                // no num_fail information
                if test_flag & 0x54 != 0 || test_flag == 0 {
                    // test passed
                    0.0f32
                } else {
                    // test failed, but no num_fail information
                    text = failed_string.to_string();
                    -1.0f32
                }
            };

            text = format!("{:04}  {}{}  {}  {}", part_idx, text, rec.vect_nam, rec.num_fail, rec.test_txt);
            Some(WorkerMessage {
                _stdf_filename: stdf_filename.to_owned(),
                log_entry: text,
                _result: result,
            })
        },
        _ => { None }
    }
}

fn stdf_worker(stdf_filename: &String, tx: Sender<WorkerMessage>, rx: Receiver<bool>) {
    let mut parser = StdfParser::new(stdf_filename, &None).unwrap();
    let mut site_to_part_idx = HashMap::<u8, u32>::new();
    let mut part_idx = 1;

    loop {
        // Loop indefinitely
        // Match the next record from the parser
        match parser.next() {
            // For all other record types, do nothing
            Ok((StdfRecord::MRR(_), _)) => {
                break;
            },
            Ok((StdfRecord::PIR(rec), _)) => {
                // update site_to_part_idx
                let site_num = rec.site_num;
                site_to_part_idx.insert(site_num, part_idx);
                part_idx = part_idx + 1;
            },
            Ok((rec, _)) => {
                // add to log
                if let Some(msg) = rec_to_worker_message(&stdf_filename, &site_to_part_idx, &rec) {
                    tx.send(msg).unwrap();
                }
            },
            Err(_) => {break},
        }

        if let Ok(should_break) = rx.try_recv() {
            if should_break { break; }
        }
    }
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<impl Write>>, Box<dyn Error>> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?; //EnableMouseCapture
    let backend = CrosstermBackend::new(stderr());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(stderr(), LeaveAlternateScreen)?; //DisableMouseCapture
    Ok(())
}

fn handle_list_key(key: event::KeyEvent, app: &mut App) -> bool {
    let mut should_break = false;
    // list active widget key handling
    match key.code {
        event::KeyCode::Char('?') => {
            // app.show_help = !app.show_help;
        },
        event::KeyCode::Char('f') => {
            app.active_widget = app::ActiveWidget::Filter;
        },
        event::KeyCode::Char('s') => {
            app.active_widget = app::ActiveWidget::Search;
        },
        event::KeyCode::Char('e') => {
            app.save_to_log(app.stdf_filename.to_owned() + ".txt").unwrap();
        },
        event::KeyCode::Char('a') => {
            app.toggle_auto_scroll();
        },
        event::KeyCode::Char('g') => {
            if app.current_layout == app::CurrentLayout::List {
                app.current_layout = app::CurrentLayout::Plot;
            } else {
                app.current_layout = app::CurrentLayout::List;
            }
        },
        event::KeyCode::Char('L') => {
            if app.graph_yscale == app::GraphYScale::Linear {
                app.graph_yscale = app::GraphYScale::Log10;
            } else {
                app.graph_yscale = app::GraphYScale::Linear;
            }
        },
        event::KeyCode::Char('n') => {
            // search for next match
            if app.search_string.len() > 0 {
                let idx = app.selected_line + 1;
                let mut iter = app.filtered_log_data.iter().skip(idx);
                if app.search_string.starts_with("/") {
                    if let Ok(re) = regex::Regex::new(&app.search_string.trim_start_matches('/')) {
                        let pos = iter.position(|x| re.is_match(&x));
                        if let Some(pos) = pos {
                            app.selected_line = idx + pos;
                        }
                    }
                } else {
                    let pos = iter.position(|x| x.contains(&app.search_string));
                    if let Some(pos) = pos {
                        app.selected_line = idx + pos;
                    }
                }
            }
        },
        event::KeyCode::Char('N') => {
            // search for next match
            if app.search_string.len() > 0 {
                let idx = app.selected_line;
                let mut iter = app.filtered_log_data.iter().take(idx).rev();
                if app.search_string.starts_with("/") {
                    if let Ok(re) = regex::Regex::new(&app.search_string.trim_start_matches('/')) {
                        let pos = iter.position(|x| re.is_match(&x));
                        if let Some(pos) = pos {
                            app.selected_line = idx - pos - 1;
                        }
                    }
                } else {
                    let pos = iter.position(|x| x.contains(&app.search_string));
                    if let Some(pos) = pos {
                        app.selected_line = idx - pos - 1;
                    }
                }
            }
        },
        event::KeyCode::Char('q') => {
            should_break = true;
        },
        event::KeyCode::Enter => {
            if app.active_widget == app::ActiveWidget::Help {
                app.active_widget = app::ActiveWidget::List;
            } else if app.active_widget == app::ActiveWidget::Search {
                app.active_widget = app::ActiveWidget::List;
            } else if app.active_widget == app::ActiveWidget::Filter {
                app.active_widget = app::ActiveWidget::List;
            }
        },
        event::KeyCode::Esc => {
            if app.active_widget == app::ActiveWidget::Help {
                app.active_widget = app::ActiveWidget::List;
            } else if app.active_widget == app::ActiveWidget::Search {
                // restore search to old state
                app.active_widget = app::ActiveWidget::List;
            } else if app.active_widget == app::ActiveWidget::Filter {
                // restore filter to old state
                app.active_widget = app::ActiveWidget::List;
            }
        },
        event::KeyCode::Up | event::KeyCode::Char('j') => {
            if app.selected_line > 0 {
                app.selected_line -= 1;
            }
        },
        event::KeyCode::Down | event::KeyCode::Char('k') => {
            if app.selected_line < app.log_data.len() - 1 {
                app.selected_line += 1;
            }
        },
        event::KeyCode::End => {
            app.selected_line = app.log_data.len() - 1;
        },
        event::KeyCode::Home => {
            app.selected_line = 0;
        },
        event::KeyCode::PageDown => {
            #[warn(unused_assignments)]
            let (mut start, mut end) = app.lines_to_display;
            let display_size = end - start;
            if display_size > app.log_data.len() - 1 - end {
                start = app.log_data.len() - display_size - 1;
            } else {
                start = end;
            }
            end = start + display_size;
            app.lines_to_display = (start, end);
            app.selected_line = end;
        },
        event::KeyCode::PageUp => {
            let (mut start, mut end) = app.lines_to_display;
            let display_size = end - start;
            if start < display_size {
                start = 0;
            } else {
                start = start - display_size;
            }
            end = start + display_size;
            app.lines_to_display = (start, end);
            app.selected_line = start;
        },
        _ => {}
    }

    should_break
}

fn handle_filter_key(key: event::KeyEvent, prv_filter: &String, app: &mut App) -> bool {
    let restore_widget = app::ActiveWidget::List;
    app.data_is_dirty = true;

    match key.code {
        event::KeyCode::Up => {
            app.filter_string = prv_filter.to_owned();
        },
        event::KeyCode::Char(c) => {
            app.filter_string.push(c);
        },
        event::KeyCode::Backspace => {
            app.filter_string.pop();
        },
        event::KeyCode::Enter => {
            app.active_widget = restore_widget;
        },
        event::KeyCode::Esc => {
            app.filter_string = prv_filter.to_owned();
            app.active_widget = restore_widget;
        }
        _ => {
            app.data_is_dirty = false;
        }
    }

    return false;
}

fn handle_search_key(key: event::KeyEvent, prv_search: &String, prv_selected: usize, app: &mut App) -> bool {
    let restore_widget = app::ActiveWidget::List;
    match key.code {
        event::KeyCode::Char(c) => {
            app.search_string.push(c);
            if !app.search_string.is_empty() && app.search_string.starts_with("/") {
                if let Ok(re) = regex::Regex::new(&app.search_string.trim_start_matches('/')) {
                    app.selected_line = app.filtered_log_data.iter().position(|x| re.is_match(&x)).unwrap_or(prv_selected);
                }
            } else {
                app.selected_line = app.filtered_log_data.iter().position(|x| x.contains(&app.search_string)).unwrap_or(prv_selected);
            }
        },
        event::KeyCode::Backspace => {
            app.search_string.pop();
            if app.search_string.is_empty() {
                app.selected_line = prv_selected;
            } else if app.search_string.starts_with("/") {
                if let Ok(re) = regex::Regex::new(&app.search_string.trim_start_matches('/')) {
                    app.selected_line = app.filtered_log_data.iter().position(|x| re.is_match(&x)).unwrap_or(prv_selected);
                }
            } else {
                app.selected_line = app.filtered_log_data.iter().position(|x| x.contains(&app.search_string)).unwrap_or(prv_selected);
            }
        },
        event::KeyCode::Enter => {
            app.active_widget = restore_widget;
        },
        event::KeyCode::Esc => {
            app.search_string = prv_search.to_owned();
            app.selected_line = prv_selected;
            app.active_widget = restore_widget;
        }
        _ => {}
    }

    return false;
}

fn run_app(rx: Receiver<WorkerMessage>, tx: Sender<bool>, stdf_filename: &String) -> Result<(), String> {
    // open file for logging
    let mut terminal = init_terminal().map_err(|e| e.to_string())?;

    let mut app = App::new();
    app.stdf_filename = stdf_filename.to_owned();
    let mut prv_filter = app.filter_string.clone();
    let mut prv_search = app.search_string.clone();
    let mut prv_selected = 0;
    let mut prv_size = terminal.size().map_err(|e| e.to_string())?;
    
    loop {
        let cur_size = terminal.size().map_err(|e| e.to_string())?;
        if app.needs_refresh || cur_size != prv_size {
            // draw everything
            prv_size = cur_size;
            terminal.draw(|f| ui(f, &mut app)).map_err(|e| e.to_string())?;
            app.needs_refresh = false;
        }

        // grab all pending messages and add them to the log
        while let Ok(msg) = rx.try_recv() {
            app.log_data.push(msg.log_entry);
            
            app.needs_refresh = true;
            app.data_is_dirty = true;
        }

        // wait for event for remainder of tick rate
        if crossterm::event::poll(Duration::from_millis(0)).map_err(|e| e.to_string())? {
            if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                app.needs_refresh = true;
    
                // handle key events for list widget, if list widget is active
                //  if handle_list_key returns true it means it's time to break the loop
                match app.active_widget {
                    ActiveWidget::List => {
                        let should_break = handle_list_key(key, &mut app);
                        if app.active_widget == ActiveWidget::Filter {
                            prv_filter = app.filter_string.clone();
                            app.filter_string = "".to_string();
                        } else if app.active_widget == ActiveWidget::Search {
                            prv_search = app.search_string.clone();
                            prv_selected = app.selected_line;
                            app.search_string = "".to_string();
                        }
                        if true == should_break {
                            // terminate worker if running
                            let _ = tx.send(true);
                            break;
                        }
                    },
                    ActiveWidget::Filter => {
                        handle_filter_key(key, &prv_filter, &mut app);
                    },
                    ActiveWidget::Search => {
                        handle_search_key(key, &prv_search, prv_selected, &mut app);
                    },
                    _ => {}
                }
            }
        }
    }

    restore_terminal().map_err(|e| e.to_string())?;

    terminal.show_cursor().map_err(|e| e.to_string())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Call the function to parse the arguments
    let args = parse_arguments();
    
    let (tx, rx) = std::sync::mpsc::channel::<WorkerMessage>();
    let (terminate_tx, terminate_rx) = std::sync::mpsc::channel::<bool>();
    let stdf_filename = args.stdf_filename.clone();

    let app_handle = thread::spawn(move || {
        run_app(rx, terminate_tx, &stdf_filename)
    });

    let stdf_filename = args.stdf_filename.clone();
    let worker_handle = thread::spawn(move || {
        stdf_worker(&stdf_filename, tx, terminate_rx)
    });

    app_handle.join().unwrap().map_err(|e| e.to_string())?;
    worker_handle.join().unwrap();

    Ok(())
}