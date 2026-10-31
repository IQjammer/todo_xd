use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub fn render_input_popup(text: &str, area: Rect, buf: &mut Buffer) {
    let popup_area = centered_rect(60,20, area);

    Clear.render(popup_area, buf);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::THICK)
        .title(" New TODO".bold())
        .border_style(Style::default());

    let display_text = format!("{}_", text);

    Paragraph::new(display_text)
        .centered()
        .block(input_block)
        .render(popup_area, buf);

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let [vertical] = Layout::vertical([Constraint::Percentage(percent_y)])
            .flex(Flex::Center)
            .areas(r);

        let [horizontal] = Layout::horizontal([Constraint::Percentage(percent_x)])
            .flex(Flex::Center)
            .areas(vertical);

        horizontal
    }

}
