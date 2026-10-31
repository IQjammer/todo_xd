use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn render_menu_list(
    items: &[String],
    selected_index: Option<usize>,
    area: Rect,
    buf: &mut Buffer,
) {
    let constraints = vec![Constraint::Length(3); items.len()];
    let chunks = Layout::vertical(constraints).split(area);

    for (i, (text, chunk)) in items.iter().zip(chunks.iter()).enumerate() {
        let is_selected = selected_index == Some(i);

        let border_color = if is_selected {
            Color::Green
        } else {
            Color::White
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .border_style(Style::default().fg(border_color));

        let mut paragraph = Paragraph::new(text.as_str()).centered().block(block);

        if is_selected {
            paragraph = paragraph.bold();
        }

        paragraph.render(*chunk, buf);
    }
}
