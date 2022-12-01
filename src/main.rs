use clap::Parser;
use chrono;
use std::{io, thread, time::Duration};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal, Frame
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

static DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

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

fn ask_user_for_input() -> std::string::String {
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut user_input)
        .expect("Something went wrong.");

    return user_input;
}

// fn create_terminal() -> Result<(), io::Error> {
//     let stdout = io::stdout();
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     Ok(())
// }

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


    thread::sleep(Duration::from_millis(5000));

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

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
         .direction(Direction::Vertical)
         .margin(1)
         .constraints(
             [
                 Constraint::Percentage(80),
                 Constraint::Percentage(20)
             ].as_ref()
         )
         .split(f.size());
     let block = Block::default()
          .title("Calendar")
          .borders(Borders::ALL);
     f.render_widget(block, chunks[0]);
     let block = Block::default()
          .title("Input")
          .borders(Borders::ALL);
     f.render_widget(block, chunks[1]);
 }

fn main() {
    let args = Cli::parse();

    // Make placeholder for days
    let my_vec: Vec<u32> = (1..=31).step_by(1).collect();
    create_terminal();

}
