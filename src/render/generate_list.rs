use ratatui::{
    layout::Rect,
    style::{Color::Green, Style, Stylize},
    widgets::{HighlightSpacing, List, ListDirection, ListItem},
};
use unicode_width::UnicodeWidthStr;

pub fn generate_list(values: &[String], area: Rect) -> List {
    let map: Vec<ListItem> = values
        .iter()
        .map(|song| {
            let max = area.width.saturating_sub(10) as usize;
            let title = if song.width() < max {
                song.to_string()
            } else {
                format!(
                    "{}...",
                    song.chars()
                        .take(max.saturating_sub(3))
                        .collect::<String>()
                        .as_str()
                )
            };
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
