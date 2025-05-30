use ratatui::{
    layout::{Constraint, Layout}, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::{Paragraph, Widget}
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);
        self.render_bottom_bar(layout[1], buf);
        let columns = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(3),
            Constraint::Length(1),
            Constraint::Fill(2),
        ])
        .split(layout[0]);
        self.render_directory_column(columns[0], buf, 0);
        self.render_directory_column(columns[2], buf, 1);
    }
}

impl App {
    fn render_bottom_bar(&self, area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer)
    {
        Line::from(self.current_paths[1].clone().unwrap()).render(area, buf);
    }

    fn render_directory_column(&self, area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer, column: usize)
    {
        let position = match column {
            0 => self.prev_position,
            1 => self.position, _ => 0
        };
        let list = &self.stored_lists[column];
        if list.0 && list.1.is_some() { // Directory is in filesystem and loaded
            let rows = list.1.as_ref().unwrap();
            let mut lines: Vec<Line> = vec![];
            let start_offset = (area.height as usize) / 2;
            let mut offset: usize;
            if position >= start_offset {
                offset = position - start_offset;
                if rows.len() > start_offset {
                    if position > rows.len() - start_offset - 1 {
                        offset = rows.len() - area.height as usize
                    }
                }
            } else {
                offset = 0;
            }
            let end = if rows.len() >= area.height as usize {
                area.height as usize + offset 
            } else {
                rows.len()
            };
            for row in rows[offset..end].iter().enumerate() {
                let mut fragments: Vec<Span> = vec![];
                if column == 0 {
                    if position - offset == row.0 {
                        fragments.push(Span::from("> ").green());
                    } else {
                        fragments.push(Span::from("  "));
                    }
                }
                if column == 1 { // Numbers are shown only for center column
                    if position - offset == row.0 {
                        fragments.push(Span::from(">> ").green());
                    }
                    if row.0 < position - offset {
                        let off = position - offset - row.0;
                        let num: String;
                        if off > 9 {
                            num = String::from(format!("{off} "))
                        } else {
                            num = String::from(format!(" {off} "))
                        }
                        fragments.push(Span::from(num));
                    } else if row.0 > position - offset {
                        let off = row.0 - (position - offset);
                        let num: String;
                        if off > 9 {
                            num = String::from(format!("{off} "))
                        } else {
                            num = String::from(format!(" {off} "))
                        }
                        fragments.push(Span::from(num));
                    }
                }
                if self.show_icons {
                    fragments.push(Span::styled(&row.1.0, Style::new().fg(Color::Rgb(row.1.1.r, row.1.1.g, row.1.1.b))));
                }
                if row.1.3 {
                    fragments.push(Span::from(&row.1.2).fg(Color::Blue));
                } else {
                    fragments.push(Span::from(&row.1.2));
                }
                if row.1.4.is_some() {
                    fragments.push(Span::from(" > ".to_string() + row.1.4.as_ref().unwrap()).fg(Color::Red));
                }
                lines.push(Line::from(fragments));

            }
            Paragraph::new(lines).render(area, buf);
        }
    }
}
