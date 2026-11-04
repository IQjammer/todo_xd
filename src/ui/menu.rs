use ratatui::{
    buffer::Buffer,
    style::Style,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Widget}
};

use super::list;
use crate::{app::{App, InputMode}, ui::input};

pub fn render_menu(app: &App, area: Rect, buf: &mut Buffer) {
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::THICK)
        .title(" RustTODO ".bold())
        .title_bottom(
            Line::from(" [h/l] to done | [j/k] to up/down | [n] to new ")
                    .style(Style::default())
                    .alignment(Alignment::Center),
        )
        .title_top(
            Line::from(" [q] to quit ")
                    .style(Style::default())
                    .alignment(Alignment::Right),
        );

    let inner_area = main_block.inner(area);
    main_block.render(area, buf);

    list::render_menu_list(
        &app.list.items,
        app.list.list_state.selected(),
        inner_area,
        buf,
    );

    if app.mode == InputMode::Editing {
        input::render_input_popup(&app.list.input, area, buf);
    }
}
