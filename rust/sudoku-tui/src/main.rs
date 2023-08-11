use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{block::BorderType, Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{io, time::Duration};
use sudoku::Sudoku;

pub mod board;
use crate::board::Board;

fn main() -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;

    let mut generated_sudoku = Board::from_bytes(Sudoku::generate_unique().to_bytes());
    let mut input_pos: (usize, usize) = (0, 0);
    let mut mark_mode = false;

    let mut game_loop = true;
    while game_loop {
        terminal.draw(|f| draw_sudoku(f, &generated_sudoku, input_pos, mark_mode))?;

        if event::poll(Duration::from_millis(17))? {
            if let Event::Key(key) = event::read()? {
                handle_input(
                    key,
                    &mut generated_sudoku,
                    &mut game_loop,
                    &mut input_pos,
                    &mut mark_mode,
                );
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

fn handle_input(
    key: KeyEvent,
    sudoku: &mut Board,
    game_loop: &mut bool,
    input_pos: &mut (usize, usize),
    mark_mode: &mut bool,
) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => *game_loop = false,
        KeyCode::Char('w') | KeyCode::Char('k') | KeyCode::Up => {
            if input_pos.0 == 0 {
                input_pos.0 = 8;
            } else {
                input_pos.0 -= 1;
            }
        }
        KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => {
            if input_pos.1 == 0 {
                input_pos.1 = 8;
            } else {
                input_pos.1 -= 1;
            }
        }
        KeyCode::Char('s') | KeyCode::Char('j') | KeyCode::Down => {
            if input_pos.0 == 8 {
                input_pos.0 = 0;
            } else {
                input_pos.0 += 1;
            }
        }
        KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
            if input_pos.1 == 8 {
                input_pos.1 = 0;
            } else {
                input_pos.1 += 1;
            }
        }
        KeyCode::Char('m') => *mark_mode = !*mark_mode,
        KeyCode::Char('c') => sudoku.boxes[input_pos.0][input_pos.1].set_value(0),
        KeyCode::Char(n) => match n {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let parsed_num: u8 = n.to_digit(10).unwrap_or(0) as u8;
                if *mark_mode && parsed_num != 0 {
                    sudoku.boxes[input_pos.0][input_pos.1].toggle_mark(parsed_num as usize);
                } else {
                    sudoku.set_box(input_pos.0, input_pos.1, parsed_num);
                    let validity = sudoku.check_validity(input_pos.0, input_pos.1);
                    sudoku.boxes[input_pos.0][input_pos.1].set_valid(validity);
                }
            }
            _ => *game_loop = true,
        },
        _ => *game_loop = true,
    }
}

// draws the current sudoku board
fn draw_sudoku<B: Backend>(
    f: &mut Frame<B>,
    sudoku: &Board,
    input_pos: (usize, usize),
    mark_mode: bool,
) {
    let sudoku_board = Rect::new(
        (f.size().width / 2) - 31,
        (f.size().height / 2) - 22,
        63,
        45,
    );

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
                Constraint::Ratio(1, 9),
            ]
            .as_ref(),
        )
        .split(sudoku_board);

    let mut boxes = Vec::new();

    for c in 0..9 {
        boxes.push(
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Ratio(1, 9),
                        Constraint::Ratio(1, 9),
                        Constraint::Ratio(1, 9),
                        Constraint::Ratio(1, 9),
                        Constraint::Ratio(1, 9),
                        Constraint::Ratio(1, 9),
                        Constraint::Ratio(1, 9),
                        Constraint::Ratio(1, 9),
                        Constraint::Ratio(1, 9),
                    ]
                    .as_ref(),
                )
                .split(rows[c]),
        )
    }

    for row in 0..9 {
        for col in 0..9 {
            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            if row == input_pos.0
                || col == input_pos.1
                || (row / 3, col / 3) == (input_pos.0 / 3, input_pos.1 / 3)
            {
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
            let box_num;
            if sudoku.boxes[row][col].value == 0 {
                let mut curr_style = Style::default();
                if sudoku.boxes[row][col].has_marks() {
                    curr_style = curr_style.fg(Color::LightYellow);
                }
                box_num = Paragraph::new(sudoku.boxes[row][col].get_marks())
                    .style(curr_style)
                    .alignment(Alignment::Center)
                    .block(block);
            } else {
                let mut curr_style = Style::default();
                if !sudoku.boxes[row][col].original {
                    if sudoku.boxes[row][col].is_valid {
                        curr_style = curr_style.fg(Color::LightGreen);
                    } else {
                        curr_style = curr_style.fg(Color::LightRed);
                    }
                }
                box_num = Paragraph::new(format!(
                    "     \n  {}  \n     ",
                    sudoku.boxes[row][col].value
                ))
                .style(curr_style)
                .alignment(Alignment::Center)
                .block(block);
            }
            f.render_widget(box_num, boxes[row][col]);
        }
    }

    let sub_box_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(sudoku_board);

    let mut sub_boxes = Vec::new();

    for c in 0..3 {
        sub_boxes.push(
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .split(sub_box_rows[c]),
        );
    }

    for row in 0..3 {
        for col in 0..3 {
            let sub_box = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            f.render_widget(sub_box, sub_boxes[row][col]);
        }
    }

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double),
        sudoku_board,
    );
}
