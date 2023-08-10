use std::{io, thread, time::Duration};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
    Frame,
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use rand::{self, Rng};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let box_colors: Vec<Vec<ratatui::style::Color>> = vec![
        vec![Color::Red, Color::Green, Color::Blue],
        vec![Color::Cyan, Color::Magenta, Color::Yellow],
        vec![Color::LightYellow, Color::LightBlue, Color::LightRed]
    ];
    // draw to the terminal
    terminal.draw(|f| ui(f, &box_colors))?;

    // discard input events
    thread::spawn(|| loop {
        let _ = event::read();
    });

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

// draws an empty sudoku board
fn ui<B: Backend>(f: &mut Frame<B>, box_colors: &Vec<Vec<ratatui::style::Color>>) {
    let full_size = f.size();

    let sudoku_square = Rect::new((full_size.width / 2) - 31, (full_size.height / 2) - 22, 63, 45);

    let major_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
        ].as_ref())
        .split(sudoku_square);

    let mut boxes = Vec::new();

    for r in 0..9 {
        boxes.push(Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
            ].as_ref())
            .split(major_rows[r]))
    }

    for row in 0..9 {
        for col in 0..9 {
            let major_box = Block::default()
                .style(Style::default().fg(box_colors[row / 3][col / 3]))
                .borders(Borders::ALL);
            let value = Paragraph::new(format!("     \n  {}  \n     ", rand::thread_rng().gen_range(1..10)))
                .alignment(Alignment::Center)
                .block(major_box);
            f.render_widget(value, boxes[row][col])
        }
    }
}
