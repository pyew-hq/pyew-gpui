use crate::components::connection::connection_sidebar::ConnectionSideBar;
use gpui::*;

#[derive(IntoElement)]
pub struct SideBar {}

impl SideBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for SideBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let connection_sidebar = window.use_keyed_state("connection-sidebar", cx, |window, cx| {
            ConnectionSideBar::new(window, cx)
        });

        div().size_full().child(connection_sidebar)
    }
}
