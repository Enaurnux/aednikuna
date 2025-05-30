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
            KeyCode::Char('q') => {
                self.exit();
            }
            KeyCode::Char('j') => {
                self.move_down();
                self.request_list();
            }
            KeyCode::Char('k') => {
                self.move_up();
                self.request_list();
            },
            KeyCode::Char('h') => {
                if self.current_paths[0].is_some() {
                    // Shifting left column to center
                    self.stored_lists[1] = self.stored_lists[0].clone();
                    self.current_paths[1] = self.current_paths[0].clone();
                    self.position = self.prev_position;
                    self.prev_position = 0;
                    self.offset = 0;
                    // Resetting left column
                    self.stored_lists[0] = (false, None);
                    self.current_paths[0] = None;
                    self.request_list();
                }
            },
            KeyCode::Char('l') => {
                if self.current_paths[1].is_some() {
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
                        self.offset = 0;
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
