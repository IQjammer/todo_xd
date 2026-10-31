use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    widgets::{Block, Widget},
};

use super::list;
use crate::{app::{App, InputMode}, ui::input};

pub fn render_menu(app: &App, area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .border_set(border::THICK)
        .title(" RustTODO ".bold());

    let inner_area = block.inner(area);
    block.render(area, buf);

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
