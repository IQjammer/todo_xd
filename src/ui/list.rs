use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn render_menu_list(
    items: &[crate::list::TodoItem],
    selected_index: Option<usize>,
    area: Rect,
    buf: &mut Buffer,
) {
    let constraints = vec![Constraint::Length(3); items.len()];
    let chunks = Layout::vertical(constraints).split(area);

    for (i, (item, chunk)) in items.iter().zip(chunks.iter()).enumerate() {
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

        let checkbox_span = if item.completed {
            Span::styled("[x] ", Style::default().fg(Color::Green).bold())
        } else {
            Span::styled("[ ] ", Style::default().fg(Color::DarkGray))
        };

        let mut text_style = Style::default();
        if item.completed {
            text_style = text_style
                .fg(Color::DarkGray)
                .add_modifier(Modifier::CROSSED_OUT);
        }

        let text_span = Span::styled(item.text.as_str(), text_style);

        let line = Line::from(vec![checkbox_span, text_span]);

        let mut paragraph = Paragraph::new(line).centered().block(block);

        if is_selected {
            paragraph = paragraph.bold();
        }

        paragraph.render(*chunk, buf);
    }
}
