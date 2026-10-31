mod list;
mod menu;

use crate::App;
use ratatui::{buffer::Buffer, layout::Rect};

pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
    menu::render_menu(app: &App, area, buf);
}
