use std::rc::Rc;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::Text, widgets::{Block, Borders, List, ListState, Paragraph}, Frame};

use crate::app::{App, CurrentLayout, ActiveWidget};

type Rects = Rc<[Rect]>;

// we need the outer and body layouts, and we use them just not after assigned
#[allow(unused)]
struct UiLayout {
    outer_layout: Rects,
    header_layout: Rects,
    body_layout: Rects,
    footer_layout: Rects,
    inner_left: Rects,
    inner_right: Option<Rects>,
}

fn get_layout(f: &mut Frame, app: &App) -> UiLayout {
    // Create Outer Layout, split the screen into 3 parts
    //  1 is the header, it will be fixed at 3 lines
    //  2 is the body, it will be split into 2 parts
    //  3 is the footer, it will be fixed at 3 lines
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1)
        ])
        .split(f.size());

    // Create Inner Layouts
    //  1 is the header layout with 100% of the width
    //  2 is the body layout with 2 parts, each 50% of the width
    //  3 is the footer layout with 100% of the width
    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[0]);
    let body_layout = if app.current_layout == CurrentLayout::List {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(outer_layout[1])
    } else { 
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(outer_layout[1])
    };
    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(outer_layout[2]);

    // Create Inner Layouts for the body
    let inner_left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
        ])
        .split(body_layout[0]);
    let inner_right =  if app.current_layout == CurrentLayout::List {
        None
    } else {
        Some(Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Min(3),
            ])
            .split(body_layout[1]))
    };

    UiLayout {
        outer_layout,
        header_layout,
        body_layout,
        footer_layout,
        inner_left,
        inner_right,
    }
}

fn filter_and_trim_log_data(app: &mut App, display_size: u16) -> Vec<String> {
    let mut selected_line = app.selected_line;
    let (mut start, mut end) = app.lines_to_display;

    if app.data_is_dirty {
        app.data_is_dirty = false;
        if app.filter_string.starts_with("/") {
            if let Ok(re) = regex::Regex::new(&app.filter_string.trim_start_matches('/')) {
                app.filtered_log_data = app.log_data.iter()
                    .filter(|t| if app.filter_string.is_empty() { true } else { re.is_match(t.as_str()) }).map(|v| v.to_owned()).collect();
            } else {
                app.filtered_log_data.clear();
            }
        } else {
            app.filtered_log_data = app.log_data.iter()
                .filter(|t| if app.filter_string.is_empty() { true } else { t.contains(&app.filter_string) }).map(|v| v.to_owned()).collect();
        }
    }

    // if we're supposed to auto-scroll, then we should always go to the last available entry.
    if app.auto_scroll {
        selected_line = app.filtered_log_data.len() - 1;
        app.selected_line = selected_line;
    }

    // move selected line to the end if it's passed the end
    if selected_line >= app.filtered_log_data.len() {
        selected_line = app.filtered_log_data.len() - 1;
        app.selected_line = selected_line;
    }

    // if we've resized the window, then we need to adjust end-start to match display size
    if end - start != (display_size-1) as usize {
        end = start + (display_size-1) as usize;
        app.lines_to_display = (start, end);
    }

    // if selected line is below start or above end we need to move the window
    if start != 0 && selected_line < start {
        start = selected_line;
        end = start + display_size as usize;
        app.lines_to_display = (start, end);
    } else if end != app.filtered_log_data.len()-1 && selected_line >= end {
        let delta = selected_line - end;
        start = start + delta;
        end = end + delta;
        app.lines_to_display = (start, end);
    }

    // update the display
    app.display_selected_line = selected_line - start;
    
    // slice the data to the window
    let filtered_data = app.filtered_log_data.iter().skip(start).take(end - start + 1).map(|v| v.to_owned() ).collect();

    filtered_data
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let layout = get_layout(f, app);

    // Header / Footer widgets
    let header_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::Gray));
    let header_filter = if app.active_widget != ActiveWidget::Filter { 
        Paragraph::new(
            format!("FILTER: {}", app.filter_string)
        )
            .style(Style::new().fg(Color::Black).bold())
            .left_aligned()
            .block(header_block.to_owned())
    } else {
        Paragraph::new(
            format!("FILTER: {}", app.filter_string)
        )
            .style(Style::new().fg(Color::White).bold().bg(Color::Black))
            .left_aligned()
            .block(header_block.to_owned())
    };
    let header_title = Paragraph::new(
        format!("FILE: {}", app.stdf_filename)
    )
        .style(Style::new().fg(Color::Black).bold())
        .right_aligned()
        .block(header_block.to_owned());

    let footer_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::Gray));
    let footer_search = if app.active_widget != ActiveWidget::Search { 
        Paragraph::new(
            format!("SEARCH: {}", app.search_string),
            // String::new(),
        )
            .style(Style::new().fg(Color::Black).bold())
            .left_aligned()
            .block(footer_block.to_owned())
    } else {
        Paragraph::new(
            format!("SEARCH: {}", app.search_string),
            // String::new(),
        )
            .style(Style::new().fg(Color::White).bg(Color::Black).bold())
            .left_aligned()
            .block(footer_block.to_owned())
    };
    let log_data = filter_and_trim_log_data(app, layout.inner_left[0].height - 2);

    let footer_help = Paragraph::new(
        format!("q: quit  f: filter  s: search  n/N: next/prev match  e: export  g: graph toggle  L: linear/log scale  arrows: Navigate  hjkl: Navigate  Enter: Confirm"),
    )
        .style(Style::new().fg(Color::Black).bold())
        .centered()
        .block(footer_block.to_owned());

    // List Widget
    // layout.inner_left[0].height
    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));
    let mut state = ListState::default().with_selected(Some(app.display_selected_line));
    let list = List::new(
            log_data
            .iter()
            .map(|i| if i.to_lowercase().contains(" failed ") { Text::styled(i, Color::Red) } else { Text::raw(i) })
        )
        .block(list_block)
        .highlight_style(Style::new().bg(Color::DarkGray));
        // .highlight_symbol("> ");

    // graphing widgets
    if let Some(inner_right) = layout.inner_right {
        // Graph and summary widgets
        // top is graph, bottom is summary

        let graph_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black)).fg(Color::Gray);
        let graph = Paragraph::new(
            format!("GRAPH: {}", "Graph Data Here")
        )
            .style(Style::new().fg(Color::Gray))
            .block(graph_block.to_owned());
        f.render_widget(graph, inner_right[0]);

        // create and render summary widget
        let summary_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black)).fg(Color::Gray);
        let summary = Paragraph::new(
            format!("SUMMARY: {}", "Summary Data Here")
        )
            .style(Style::new().fg(Color::Gray))
            .block(summary_block.to_owned());
        f.render_widget(summary, inner_right[1]);
    }

    // Render Widgets
    f.render_widget(header_filter, layout.header_layout[0]);
    f.render_widget(header_title, layout.header_layout[1]);

    f.render_stateful_widget(list, layout.inner_left[0], &mut state);

    f.render_widget(footer_search, layout.footer_layout[0]);
    f.render_widget(footer_help, layout.footer_layout[1]);
}