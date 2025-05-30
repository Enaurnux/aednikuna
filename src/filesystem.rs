use std::fs::read_dir;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use devicons::FileIcon;
use hex_color::HexColor;

pub fn list(path: String, tx: Sender<(String, Vec<(String, HexColor, String, bool, Option<String>)>, usize)>, show_icons: bool, sel: String) {
    let mut paths: Vec<(String, HexColor, String, bool, Option<String>)> = vec![];
    let iter = match read_dir(&path) {
        Ok(v) => v,
        Err(_) => {
            match tx.send((path, vec![], 0)) {
                _ => return,
            };
        }
    };
    for entry in iter {
        let entry = entry.unwrap();
        let name = entry.file_name().display().to_string();
        let icon_symbol: String;
        let icon_color: HexColor;
        if show_icons {
            let icon = FileIcon::from(entry.path());
            icon_symbol = String::from(icon.icon.to_string() + "  ");
            icon_color = HexColor::parse(&icon.color.to_string()).unwrap()
        } else {
            icon_symbol = String::new();
            icon_color = HexColor::rgb(0, 0, 0);
        }
        let mut dir = entry.metadata().unwrap().is_dir();
        let sym = if entry.metadata().unwrap().is_symlink() {
            let link = String::from("/") + &entry.path().read_link().unwrap().display().to_string();
            if PathBuf::from(&link).is_dir() {
                dir = true;
            }
            Some(link)
        } else {
            None
        };
        paths.push((icon_symbol, icon_color, name, dir, sym));
    }
    paths.sort_by(|a, b| a.2.cmp(&b.2));
    let mut pos: usize = 0;
    for path in paths.iter().enumerate() {
        if path.1.2 == sel {
            pos = path.0;
            break;
        } else { pos = 0; }
    }
    match tx.send((path, paths, pos)) {
        _ => {}
    };
}
