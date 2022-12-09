use chrono;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, read, KeyEvent, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame, Terminal,
};


#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn write_to_file(path: std::path::PathBuf, data: String) -> Result<(), Box<dyn std::error::Error>> {
    let write = std::fs::write(path, data);

    match write {
        Ok(()) => {}
        Err(error) => {
            panic!("Could not make file! ERROR: {}", error);
        }
    };
    Ok(())
}

fn create_terminal() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default();
        //     .title("Block")
        //     .borders(Borders::ALL);
        ui(f);
        f.render_widget(block, size);
    })?;

    loop {
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            #[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => println!("{:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
            Event::Paste(_) => todo!(),
        }
    }

    // let wait_time_in_millis = time::Duration::from_millis(1000);
    // thread::sleep(wait_time_in_millis);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw_table<'a>() -> Table<'a> {
    Table::new(vec![
        Row::new(vec!["."; 7]),
        Row::new(vec!["."; 7]),
        Row::new(vec!["."; 7]),
        Row::new(vec!["."; 7]),
    ])
    // You can set the style of the entire Table.
    .style(Style::default().fg(Color::White))
    // It has an optional header, which is simply a Row always visible at the top.
    .header(
        Row::new(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
            .style(Style::default().fg(Color::Green))
            // If you want some space between the header and the rest of the rows, you can always
            // specify some margin at the bottom.
            .bottom_margin(1),
    )
    // As any other widget, a Table can be wrapped in a Block.
    .block(Block::default().title("Table").borders(Borders::ALL))
    // Columns widths are constrained in the same way as Layout...
    .widths(&[Constraint::Length(5); 7])
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1)
    // If you wish to highlight a row in any specific way when it is selected...
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>")
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(chunks[1]);

    let block = Block::default().title("Title").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    // Let's make a table block
    let block = draw_table();
    f.render_widget(block, body_chunks[0]);

    // Info block
    let block = Block::default().title("Info").borders(Borders::ALL);
    f.render_widget(block, body_chunks[1]);
}

fn main() {
    let args = Cli::parse();

    // Make placeholder for days
    let my_vec: Vec<u32> = (1..=31).step_by(1).collect();
    create_terminal();
}
