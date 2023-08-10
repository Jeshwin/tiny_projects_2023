use std::{io, time::Duration};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    widgets::{Block, block::BorderType, Borders, Paragraph},
    style::{Style, Color},
    Frame,
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use rand::{self, Rng};

fn main() -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;

    loop {
        terminal.draw(|f| ui(f))?;

        if event::poll(Duration::from_millis(17))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    }

    // // discard input events
    // thread::spawn(|| loop {
    //     let _ = event::read();
    // });

    // thread::sleep(Duration::from_millis(5000));

    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(terminal.show_cursor()?)
}

// draws an empty sudoku board
fn ui<B: Backend>(f: &mut Frame<B>) {
    let sudoku_square = Rect::new((f.size().width / 2) - 31, (f.size().height / 2) - 22, 63, 45);

    // let box_colors = vec![
    //     vec![Color::Red, Color::Green, Color::Blue],
    //     vec![Color::Cyan, Color::Magenta, Color::Yellow],
    //     vec![Color::LightYellow, Color::LightBlue, Color::LightRed]
    // ];

    let rows = Layout::default()
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

    for c in 0..9 {
        boxes.push(Layout::default()
            .direction(Direction::Horizontal)
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
            .split(rows[c]))
    }

    for row in 0..9 {
        for col in 0..9 {
            let block = Block::default()
                // .style(Style::default().fg(box_colors[row / 3][col / 3]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            let value = Paragraph::new(format!("     \n  {}  \n     ", rand::thread_rng().gen_range(1..10)))
                .alignment(Alignment::Center)
                .block(block);
            f.render_widget(value, boxes[row][col])
        }
    }

    let sub_box_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ].as_ref())
        .split(sudoku_square);

    let mut sub_boxes = Vec::new();

    for c in 0..3 {
        sub_boxes.push(Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ].as_ref())
            .split(sub_box_rows[c]));
    }

    for row in 0..3 {
        for col in 0..3 {
            let sub_box = Block::default()
                // .style(Style::default().fg(box_colors[row][col]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            f.render_widget(sub_box, sub_boxes[row][col]);
        }
    }

    f.render_widget(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double),
    sudoku_square);
}
