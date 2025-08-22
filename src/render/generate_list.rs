use ratatui::{
    layout::Rect,
    style::{Color::Green, Style},
    widgets::{HighlightSpacing, List, ListDirection, ListItem},
};
use unicode_width::UnicodeWidthStr;

pub fn generate_list(values: &[String], area: Rect, music_location: &str) -> List<'static> {
    let max = area.width.saturating_sub(10) as usize;
    let map: Vec<ListItem> = values
        .iter()
        .map(|song| {
            let mut title: String = String::new();

            if let Some(split) = song.split(music_location).last() {
                title = format_string(split, max);
            } else {
                title = format_string(song, max);
            }
            ListItem::from(title)
        })
        .collect();

    List::new(map)
        .style(Style::new().gray())
        .highlight_style(Style::default().bold().fg(Green))
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol("-> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}

fn format_string(title: &str, max: usize) -> String {
    if title.width() < max {
        title.to_string();
        let mut split: Vec<&str> = title.split(".").collect();
        if !split.is_empty() {
            split.pop();
        }
        split.join(".").to_string()
    } else {
        format!(
            "{}...",
            title
                .chars()
                .take(max.saturating_sub(3))
                .collect::<String>()
                .as_str()
        )
    }
}
