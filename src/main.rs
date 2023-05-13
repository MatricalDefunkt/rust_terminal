#![allow(unused_variables, unused_imports, unused_assignments)]
use crossterm::{
    cursor::{MoveRight, MoveUp},
    event::{
        poll, read, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton,
        MouseEvent, MouseEventKind,
    },
    terminal::enable_raw_mode,
    QueueableCommand,
};

use std::io::Write;

struct Row {
    chars: Vec<KeyCode>,
}

fn main() {
    let mut stdout = std::io::stdout();
    stdout.queue(EnableMouseCapture).unwrap();
    enable_raw_mode().unwrap();
    let mut rows: Vec<Row> = vec![];
    let mut current_char: KeyCode = KeyCode::Null;
    let mut current_row_index = 0;
    loop {
        if poll(std::time::Duration::from_millis(500)).unwrap() {
            match read().unwrap() {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char(c) => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    match c {
                                        'c' => {
                                            println!("Exiting...");
                                            break;
                                        }
                                        's' => {
                                            println!("Saving...");
                                        }
                                        _ => {}
                                    }
                                } else {
                                    if rows.is_empty() {
                                        rows.push(Row { chars: vec![] });
                                    }
                                    rows[current_row_index].chars.push(KeyCode::Char(c));
                                    current_char = KeyCode::Char(c);
                                    print!("{}", c);
                                    stdout.flush().unwrap();
                                }
                            }
                            KeyCode::Enter => {
                                current_char = KeyCode::Null;
                                rows.push(Row { chars: vec![] });
                                current_row_index += 1;
                                print!("\n");
                            }
                            KeyCode::Backspace => {
                                if rows[current_row_index].chars.len() > 0 {
                                    rows[current_row_index].chars.pop();
                                    current_char = match rows[current_row_index].chars.last() {
                                        Some(c) => c.clone(),
                                        None => KeyCode::Null,
                                    };
                                    print!("\x08 \x08");
                                    stdout.flush().unwrap();
                                } else {
                                    if rows[current_row_index].chars.is_empty()
                                        && current_row_index != 0
                                    {
                                        rows.pop();
                                        current_row_index -= 1;
                                        current_char = match rows[current_row_index].chars.last() {
                                            Some(c) => c.clone(),
                                            None => KeyCode::Null,
                                        };

                                        stdout.queue(MoveUp(1)).unwrap();
                                        stdout
                                            .queue(MoveRight(
                                                if rows[current_row_index].chars.len() > 1 {
                                                    rows[current_row_index].chars.len() as u16
                                                } else {
                                                    0
                                                },
                                            ))
                                            .unwrap();
                                        stdout.flush().unwrap();
                                    }
                                }
                            }
                            KeyCode::Esc => {
                                // Pause thread on pressing escape; dummy code for breakpoint
                                _ = 1 + 2;
                            }
                            _ => {}
                        }
                    }
                }
                // Event::Mouse(MouseEvent {
                //     kind, row, column, ..
                // }) => {
                //     if kind == MouseEventKind::Down(MouseButton::Right)
                //         || kind == MouseEventKind::Down(MouseButton::Left)
                //     {
                //         println!("Mouse clicked at ({}, {})", row, column);
                //     }

                //     if kind == MouseEventKind::Drag(MouseButton::Left) {
                //         println!("({}, {})", row, column);
                //     }
                // }
                _ => {}
            }
        }
    }
}
