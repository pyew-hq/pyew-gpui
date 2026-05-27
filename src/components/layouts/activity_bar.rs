use crate::{theme, utils::app_icon::AppIcon};
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    ActiveTheme, Icon, Sizable,
};

#[derive(IntoElement)]
pub struct ActivityBar {}

impl ActivityBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for ActivityBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .px_2()
            .pb_0p5()
            .flex()
            .items_center()
            .justify_between()
            .bg(theme::colors::BACKGROUND)
            .text_xs()
            .text_color(theme::colors::TEXT_MUTED)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        Button::new("activity-database").ghost().small().icon(
                            Icon::new(AppIcon::Database).text_color(cx.theme().muted_foreground),
                        ),
                    )
                    .child(
                        Button::new("activity-history").ghost().small().icon(
                            Icon::new(AppIcon::History).text_color(cx.theme().muted_foreground),
                        ),
                    )
                    .child(
                        Button::new("activity-saved-query")
                            .ghost()
                            .small()
                            .icon(Icon::new(AppIcon::Save).text_color(cx.theme().muted_foreground)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    // .child(
                    //     Button::new("language")
                    //         .ghost()
                    //         .small()
                    //         .text_color(cx.theme().muted_foreground)
                    //         .label("SQL")
                    //         .icon(Icon::new(AppIcon::Code)),
                    // )
                    .child("0 cells")
                    .child("0 cols")
                    // .child("Count 0")
                    .child("Sum 0") // .child("Avg 0"),
                    .child(
                        Button::new("AI")
                            .ghost()
                            .small()
                            .icon(Icon::new(AppIcon::Ai).text_color(cx.theme().muted_foreground)),
                    ),
            )
    }
}
