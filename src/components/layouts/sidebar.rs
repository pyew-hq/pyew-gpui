use crate::theme;
use gpui::*;
use gpui_component::{
    h_flex,
    list::ListItem,
    tree::{tree, TreeItem, TreeState},
    ActiveTheme as _, IconName, StyledExt,
};

#[derive(IntoElement)]
pub struct SideBar {
    title: SharedString,
}

impl SideBar {
    pub fn new(title: SharedString) -> Self {
        Self { title }
    }
}

fn file_tree_items() -> Vec<TreeItem> {
    vec![
        TreeItem::new("src", "src")
            .expanded(true)
            .child(
                TreeItem::new("src/components", "components")
                    .expanded(true)
                    .child(
                        TreeItem::new("src/components/layouts", "layouts")
                            .child(TreeItem::new(
                                "src/components/layouts/sidebar.rs",
                                "sidebar.rs",
                            ))
                            .child(TreeItem::new(
                                "src/components/layouts/titlebar.rs",
                                "titlebar.rs",
                            )),
                    )
                    .child(TreeItem::new("src/components/mod.rs", "mod.rs")),
            )
            .child(
                TreeItem::new("src/theme", "theme")
                    .child(TreeItem::new("src/theme/colors.rs", "colors.rs"))
                    .child(TreeItem::new("src/theme/heights.rs", "heights.rs")),
            )
            .child(
                TreeItem::new("src/window", "window")
                    .child(TreeItem::new("src/window/root_window.rs", "root_window.rs")),
            )
            .child(TreeItem::new("src/main.rs", "main.rs")),
        TreeItem::new("Cargo.toml", "Cargo.toml"),
        TreeItem::new("README.md", "README.md"),
    ]
}

impl RenderOnce for SideBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let tree_state = window.use_keyed_state("sidebar-file-tree", cx, |_, cx| {
            TreeState::new(cx).items(file_tree_items())
        });

        div()
            .size_full()
            .p_2()
            .bg(theme::colors::CARD)
            .rounded_lg()
            .text_color(theme::colors::TEXT)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .gap_2()
                    .child(
                        div()
                            .py_1()
                            .text_color(cx.theme().muted_foreground)
                            .font_bold()
                            .border_b_1()
                            .border_color(cx.theme().muted)
                            .child(self.title),
                    )
                    .child(
                        tree(&tree_state, |ix, entry, selected, _window, cx| {
                            let item = entry.item();
                            let icon = if !entry.is_folder() {
                                IconName::File
                            } else if entry.is_expanded() {
                                IconName::FolderOpen
                            } else {
                                IconName::Folder
                            };
                            // let item_id = item.id.clone();
                            let item_label = item.label.clone();

                            ListItem::new(ix)
                                .w_full()
                                .rounded(cx.theme().radius)
                                .px_2()
                                .text_color(cx.theme().muted_foreground)
                                .pl(px(16.0) * entry.depth() + px(8.0))
                                .selected(selected)
                                .font_semibold()
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .items_center()
                                        .child(icon)
                                        .child(item_label),
                                )
                                .on_click(move |_, _, _| {
                                    // println!("Selected file tree item: {}", item_id);
                                })
                        })
                        .size_full(),
                    ),
            )
    }
}
