use crate::components::layouts::main_panel::MainPanel;
use crate::components::layouts::sidebar::SideBar;
use crate::components::layouts::titlebar::TitleBar;
use crate::theme;
use gpui::*;

use gpui_component::resizable::{h_resizable, resizable_panel};

pub struct RootWindow {
    title: SharedString,
}

impl RootWindow {
    pub fn new() -> Self {
        Self {
            title: "Pyew".into(),
        }
    }
}

impl Render for RootWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme::colors::BACKGROUND)
            .rounded_lg()
            .child(TitleBar::new(self.title.clone()))
            .child(
                div().size_full().p_1().pt_0().child(
                    h_resizable("main-layout")
                        .child(
                            resizable_panel()
                                .w_1_4()
                                .mr_1()
                                .child(SideBar::new("Databases".into())),
                        )
                        .child(
                            div()
                                .size_full()
                                .child(MainPanel::new(self.title.clone()))
                                .into_any_element(),
                        ),
                ),
            )
    }
}
