use std::{path::PathBuf, sync::mpsc, thread};
use crate::{app::App, filesystem::list};

impl App {
    pub fn request_list(&mut self) {
        let path = self.current_paths[1].as_ref().unwrap().clone();
        if self.stored_lists[1].1.is_none() && !self.requested_lists.contains_key(&path) {
            let (tx, rx) = mpsc::channel();
            self.requested_lists.insert(path.clone(), rx);
            let npath = path.clone();
            let show_icons = self.show_icons;
            self.stored_lists[1].0 = true;
            thread::spawn(move || list(npath, tx, show_icons, String::new()));
        }

        let mut prev = PathBuf::from(&path);
        if prev.pop() {
            let prev = prev.display().to_string();
            if self.stored_lists[0].1.is_none() && !self.requested_lists.contains_key(&prev) {
                let (tx, rx) = mpsc::channel();
                self.requested_lists.insert(prev.clone(), rx);
                let npath = prev.clone();
                let show_icons = self.show_icons;
                thread::spawn(move || list(npath, tx, show_icons, path.split("/").last().unwrap().to_string()));
            }
            self.current_paths[0] = Some(prev);
            self.stored_lists[0].0 = true;
        } else {
            self.current_paths[0] = None;
            self.stored_lists[0].0 = false;
        }
    }

    pub fn handle_tasks(&mut self) {
        let mut for_removal: Vec<String> = vec![];
        for i in self.requested_lists.keys() {
            match self.requested_lists.get(i).unwrap().try_recv() {
                Ok(v) => {
                    let mut ind = 0;
                    for j in &self.current_paths {
                        if j.is_some() {
                            let path = j.as_ref().unwrap().to_owned();
                            if path == v.0 {
                                self.stored_lists[ind].1 = Some(v.1);
                                if ind == 0 {
                                    self.prev_position = v.2;
                                }
                                break;
                            }
                        }
                        ind += 1;
                    }
                    for_removal.push(i.to_owned());
                }
                Err(_) => {}
            }
        }
        for i in for_removal {
            self.requested_lists.remove(&i);
        }
    }

    pub fn get_preview(&mut self) {
        if self.stored_lists[1].1.is_some() {
            
        }
    }
}
