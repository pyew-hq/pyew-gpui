mod components;
mod state;
mod theme;
mod utils;
mod window;

use gpui::*;
use gpui_component::Root;
use window::root_window::RootWindow;

use crate::{state::app_state::AppState, utils::local_data::initialize_local_db};

async fn init_db(state: AppState) {
    if let Ok(db_conn) = initialize_local_db().await {
        state.set_app_db_connection(db_conn).await;
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    let state = AppState::new();
    let state_for_db = state.clone();

    app.run(move |cx| {
        gpui_component::init(cx);
        theme::apply_component_theme(cx);

        // Initialize AppState as a Global
        cx.set_global(state);

        // Spawn task to initialize the database
        cx.background_executor().spawn(init_db(state_for_db.clone())).detach();

        // Open the root window
        cx.open_window(WindowOptions::default(), |window, cx| {
            let view = cx.new(|_| RootWindow::new());
            cx.new(|cx| Root::new(view, window, cx).window_shadow_size(px(0.0)))
        })
        .expect("Failed to open window");
    });
}
