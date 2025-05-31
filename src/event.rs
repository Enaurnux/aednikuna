use crate::app::App;
use std::{io, path::PathBuf, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, poll, read};

impl App {
    pub fn handle_events(&mut self) -> io::Result<()> {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        } 
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.key_main(key_event);
    }

    fn key_main(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') |
            KeyCode::Char('4') | KeyCode::Char('5') | KeyCode::Char('6') |
            KeyCode::Char('7') | KeyCode::Char('8') | KeyCode::Char('9') |
            KeyCode::Char('0') => {
                self.number_buffer += &key_event.code.as_char().unwrap().to_string();
                log::info!("{}", self.number_buffer);
            }
            KeyCode::Char('q') => {
                self.exit();
            }
            KeyCode::Char('j') => {
                let mut count: usize = 1;
                if !self.number_buffer.is_empty() {
                    count = self.number_buffer.parse::<usize>().unwrap();
                }
                self.number_buffer = String::new();
                for _ in 0..count {
                    self.move_down();
                }
            }
            KeyCode::Char('k') => {
                let mut count: usize = 1;
                if !self.number_buffer.is_empty() {
                    count = self.number_buffer.parse::<usize>().unwrap();
                }
                self.number_buffer = String::new();
                for _ in 0..count {
                    self.move_up();
                }
            },
            KeyCode::Char('d') => {
                let mut count: usize = 1;
                if !self.number_buffer.is_empty() {
                    count = self.number_buffer.parse::<usize>().unwrap();
                }
                self.number_buffer = String::new();
                let half = (self.current_rect.height as usize - 1) / 2;
                for _ in 0..count {
                    for _ in 0..half {
                        self.move_down();
                    }
                }
            },
            KeyCode::Char('u') => {
                let mut count: usize = 1;
                if !self.number_buffer.is_empty() {
                    count = self.number_buffer.parse::<usize>().unwrap();
                }
                self.number_buffer = String::new();
                let half = (self.current_rect.height as usize - 1) / 2;
                for _ in 0..count {
                    for _ in 0..half {
                        self.move_up();
                    }
                }
            },
            KeyCode::Char('h') => {
                if self.current_paths[0].is_some() {
                    // Shifting left column to center
                    self.stored_lists[1] = self.stored_lists[0].clone();
                    self.current_paths[1] = self.current_paths[0].clone();
                    self.position = self.prev_position;
                    self.prev_position = 0;
                    // Resetting left column
                    self.stored_lists[0] = (false, None);
                    self.current_paths[0] = None;
                    self.request_list();
                }
            },
            KeyCode::Char('l') => {
                if self.stored_lists[1].1.is_some() {
                    let new_path = String::from(format!("{}/{}",
                        self.current_paths[1].as_ref().unwrap(),
                        self.stored_lists[1].1.as_ref().unwrap()[self.position].2));
                    if PathBuf::from(&new_path).is_dir() {
                        // Shifting center column to left
                        self.stored_lists[0] = self.stored_lists[1].clone();
                        self.current_paths[0] = self.current_paths[1].clone();
                        // Setting new center column
                        self.stored_lists[1] = (false, None);
                        self.current_paths[1] = Some(new_path);
                        self.prev_position = self.position;
                        self.position = 0;
                        self.request_list();
                    }
                }
            }
            _ => {}
        }
    }

    fn move_down(&mut self) {
        if self.position != self.stored_lists[1].1.as_ref().unwrap().len() - 1 {
            self.position += 1;
        }
    }

    fn move_up(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }
}
