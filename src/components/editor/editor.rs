use crate::theme;
use gpui::*;
use gpui_component::input::{Input, InputState};

#[derive(IntoElement)]
pub struct Editor {
    content: SharedString,
}

impl Editor {
    pub fn new(content: SharedString) -> Self {
        Self { content }
    }
}

impl RenderOnce for Editor {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("sql") // Language for syntax highlighting
                .line_number(true) // Show line numbers
                .searchable(true) // Enable search functionality
                .show_whitespaces(true) // Show whitespace characters
                .default_value(self.content)
        });

        div()
            .size_full()
            .text_color(theme::colors::TEXT)
            .bg(theme::colors::CARD)
            .child(
                Input::new(&state)
                    .h_full()
                    .bg(theme::colors::CARD)
                    .size_full(),
            )
    }
}
