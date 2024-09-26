use std::{fs::OpenOptions, io::Write};

#[derive(PartialEq)]
pub enum CurrentLayout {
    List,
    Plot,
}

#[derive(PartialEq)]
pub enum ActiveWidget {
    List,
    Search,
    Filter,
    Help,
}

#[derive(PartialEq)]
pub enum GraphType {
    Histogram,
    Line,
}

#[derive(PartialEq)]
pub enum GraphYScale {
    Linear,
    Log10,
}

#[derive(PartialEq)]

pub struct App {
    pub current_layout: CurrentLayout,
    pub active_widget: ActiveWidget,
    pub graph_type: GraphType,
    pub graph_yscale: GraphYScale,
    pub search_string: String,
    pub filter_string: String,
    pub log_data: Vec<String>,
    pub filtered_log_data: Vec<String>,
    pub selected_line: usize,
    pub display_selected_line: usize,
    pub auto_scroll: bool,
    pub needs_refresh: bool,
    pub data_is_dirty: bool,
    pub lines_to_display: (usize, usize),
    pub stdf_filename: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_layout: CurrentLayout::List,
            active_widget: ActiveWidget::List,
            graph_type: GraphType::Line,
            graph_yscale: GraphYScale::Linear,
            search_string: String::new(),
            filter_string: String::new(),
            log_data: Vec::new(),
            filtered_log_data: Vec::with_capacity(1000),
            selected_line: 0,
            display_selected_line: 0,
            auto_scroll: false,
            needs_refresh: false,
            data_is_dirty: false,
            lines_to_display: (0, 0),
            stdf_filename: String::new(),
        }
    }

    pub fn save_to_log(&mut self, file_path: String) -> Result<(), std::io::Error> {
        // open file to save the log into
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        // write the log data to the file
        for line in &self.log_data {
            file.write(line.as_bytes())?;
            file.write(b"\n")?;
        }

        // close the file
        Ok(())
    }

    pub fn toggle_auto_scroll(&mut self) {
        self.auto_scroll = !self.auto_scroll;
    }
}
