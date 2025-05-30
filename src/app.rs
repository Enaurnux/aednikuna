use hex_color::HexColor;
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Frame};
use std::collections::HashMap;
use std::io;
use std::sync::mpsc::Receiver;

pub enum State {
    Main
}
pub struct App {
    pub requested_lists: HashMap<String, Receiver<(String, Vec<(String, HexColor, String, bool, Option<String>)>, usize)>>,
    pub stored_lists: [(bool, Option<Vec<(String, HexColor, String, bool, Option<String>)>>); 2],
    pub current_paths: [Option<String>; 2],
    pub prev_position: usize,
    pub position: usize,
    pub offset: usize,
    pub show_icons: bool,
    pub state: State,
    pub current_rect: Rect,
    exit: bool,
}

impl App {
    pub fn new(path: String) -> App {
        App {
            requested_lists: HashMap::new(),
            stored_lists: [(false, None), (false, None)],
            current_paths: [None, Some(path)],
            prev_position: 0,
            position: 0,
            offset: 0,
            show_icons: true,
            state: State::Main,
            current_rect: Rect::new(0, 0, 0, 0),
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.request_list();
        while !self.exit {
            terminal.draw(|frame| {
                self.current_rect = frame.area();
                self.draw(frame)
            })?;
            self.handle_events()?;
            self.handle_tasks();
            self.get_preview();
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
