mod components;
mod theme;
mod window;

use gpui::*;
use gpui_component::theme::Theme;
use gpui_component::Root;
use window::root_window::RootWindow;

fn main() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        let component_theme = Theme::global_mut(cx);
        component_theme.background = theme::colors::BACKGROUND.into();
        component_theme.border = Hsla::transparent_black();
        component_theme.drag_border = theme::colors::BACKGROUND.into();

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| RootWindow::new());
                cx.new(|cx| Root::new(view, window, cx).window_shadow_size(px(0.0)))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
