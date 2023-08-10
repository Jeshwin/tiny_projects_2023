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
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use sudoku::Sudoku;

fn main() -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;

    let current_sudoku = Sudoku::generate_unique();
    let mut input_pos: (usize, usize) = (0, 0);
    let mut mark_mode = false;

    let mut game_loop = true;
    while game_loop {
        terminal.draw(|f| draw_sudoku(f, current_sudoku, input_pos, mark_mode))?;

        if event::poll(Duration::from_millis(17))? {
            if let Event::Key(key) = event::read()? {
                handle_input(key, &mut game_loop, &mut input_pos, &mut mark_mode);
            }
        }
    }

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

fn handle_input(key: KeyEvent, game_loop: &mut bool, input_pos: &mut (usize, usize), mark_mode: &mut bool) {
    match key.code {
        KeyCode::Char('q') => *game_loop = false,
        KeyCode::Char('w') | KeyCode::Char('k') | KeyCode::Up => {
            if input_pos.0 == 0 {
                input_pos.0 = 8;
            } else {
                input_pos.0 -= 1;
            }
        },
        KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => {
            if input_pos.1 == 0 {
                input_pos.1 = 8;
            } else {
                input_pos.1 -= 1;
            }
        },
        KeyCode::Char('s') | KeyCode::Char('j') | KeyCode::Down => {
            if input_pos.0 == 8 {
                input_pos.0 = 0;
            } else {
                input_pos.0 += 1;
            }
        },
        KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
            if input_pos.1 == 8 {
                input_pos.1 = 0;
            } else {
                input_pos.1 += 1;
            }
        },
        KeyCode::Char('m') => *mark_mode = !*mark_mode,
        _ => *game_loop = true,
    }
}

// draws the current sudoku board
fn draw_sudoku<B: Backend>(
    f: &mut Frame<B>,
    current_sudoku: Sudoku,
    input_pos: (usize, usize),
    mark_mode: bool
    ) {
    let sudoku_array = current_sudoku.to_bytes();
    let sudoku_board = Rect::new((f.size().width / 2) - 31, (f.size().height / 2) - 22, 63, 45);

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
        .split(sudoku_board);

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
            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            if row == input_pos.0 || col == input_pos.1 || (row / 3, col / 3) == (input_pos.0 / 3, input_pos.1 / 3) {
                if (row, col) == input_pos {
                    if mark_mode {
                     block = block.style(Style::default().bg(Color::Red));
                    } else {
                        block = block.style(Style::default().bg(Color::Blue));
                    }
                } else {
                    block = block.style(Style::default().bg(Color::DarkGray));
                }
            }
            if sudoku_array[row*9+col] == 0 {
                f.render_widget(block, boxes[row][col]);
            } else {
                let value = Paragraph::new(format!("     \n  {}  \n     ", sudoku_array[row * 9 + col]))
                    .alignment(Alignment::Center)
                    .block(block);
                f.render_widget(value, boxes[row][col]);
            }
        }
    }

    let sub_box_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ].as_ref())
        .split(sudoku_board);

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
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            f.render_widget(sub_box, sub_boxes[row][col]);
        }
    }

    f.render_widget(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double),
    sudoku_board);
}
