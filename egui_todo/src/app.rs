use egui::{
    FontData,
    epaint::text::{FontInsert, InsertFontFamily},
};
use std::collections::HashMap;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct TodoItem {
    id: u32,
    name: String,
    body: String,
}

impl Default for TodoItem {
    fn default() -> Self {
        let id = chrono::Utc::now().timestamp_subsec_nanos();
        Self {
            id,
            name: format!("Todo Item {}", id),
            body: String::new(),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // If we add new fields, give them default values when deserializing old state.
pub struct TodoApp {
    label: String,

    #[serde(skip)] // Don't serialize the field.
    value: f32,

    items: HashMap<u32, TodoItem>,
    next_id: u32,
    currently_edited: Option<(u32, TodoItem)>,
}

impl Default for TodoApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.5,
            items: HashMap::default(),
            next_id: 0,
            currently_edited: None,
        }
    }
}

impl TodoApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::setup_font(&cc.egui_ctx);

        cc.egui_ctx.set_zoom_factor(1.2);

        // Load previous app state (if any).
        // Note: The `persistence` feature must be enabled for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn setup_font(ctx: &egui::Context) {
        ctx.add_font(FontInsert::new(
            "Supreme",
            FontData::from_static(include_bytes!("../assets/fonts/Supreme-Regular.ttf")),
            vec![
                InsertFontFamily {
                    family: egui::FontFamily::Proportional,
                    priority: egui::epaint::text::FontPriority::Highest,
                },
                InsertFontFamily {
                    family: egui::FontFamily::Monospace,
                    priority: egui::epaint::text::FontPriority::Lowest,
                },
            ],
        ));
    }
}

impl eframe::App for TodoApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //
        let main_window = egui::CentralPanel::default();
        main_window.show(ctx, |ui| {
            ui.add_space(16.0);
            ui.heading("Todo List");
            ui.add_space(16.0);
            ui.label("Click 'Add Item' to add items to the list. Click on an item to view, edit, or remove it. The item will open in a new window.");
            ui.add_space(10.0);

            // Create a clone of the todo list so that we can remove items while iterating.
            // This also lets us use the id and items without a reference.
            let items = self.items.clone();

            // Indent all todo items
            ui.indent("todo_items", |ui| {
                // For every item, show its name as a clickable label.
                for (id, item) in items {
                    ui.add_space(5.0);
                    if ui
                        .add(egui::Label::new(&item.name).sense(egui::Sense::click())).on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.currently_edited = Some((id, item));
                    };
                    ui.add_space(5.0);
                }
            });

            ui.add_space(20.0);

            let add_btn = egui::Button::new("Add Item");
            if ui
                .add_enabled(self.currently_edited.is_none(), add_btn)
                .clicked()
            {
                self.items.insert(self.next_id, TodoItem::default());
                self.next_id += 1;
            }

            // If we're currently editing an item, we have to keep calling ctx.show_viewport_immediate.
            // Remove the currently edited id and item, replace later if necessary.
            if let Some((id, mut item)) = self.currently_edited.take() {
                let viewport_id = egui::ViewportId::from_hash_of(format!("edit {id}"));
                let viewport_builder = egui::ViewportBuilder::default()
                    .with_inner_size((300.0, 300.0))
                    .with_title(format!("edit {}", item.name));

                // This function is like eframe::App::update, except it can access ExampleApp as well
                let viewport_cb = |ctx: &egui::Context, _| {
                    egui::Window::new(format!("{}", item.id))
                        .collapsible(false)
                        .title_bar(false)
                        // .anchor(egui::Align2::CENTER_CENTER, (0.0, 0.0))
                        // .default_pos(ctx.screen_rect().center())
                        .movable(true)
                        .resizable(false)
                        .order(egui::Order::Foreground)
                        .show(ctx, |ui| {
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut item.name);
                            ui.label("Body:");
                            ui.text_edit_multiline(&mut item.body);
                            if ui.button("Save").clicked() {
                                // Insert the updated item at the id.
                                self.items.insert(id, item.clone());

                                // Set the currently edited item to nothing.
                                self.currently_edited = None;
                            } else if ui.button("Cancel").clicked()
                                || ctx.input(|i| i.viewport().close_requested())
                            {
                                // Set the currently edited item to nothing.
                                self.currently_edited = None;
                            } else if ui.button("Remove").clicked() {
                                // Remove the currently edited item.
                                self.items.remove(&id);
                                // Set the currently edited item to nothing.
                                self.currently_edited = None;
                                log::info!("Removed item with id {}", id);
                            } else {
                                // Otherwise set the currently edited item to this item again,
                                // so the window won't close.
                                self.currently_edited = Some((id, item.clone()));
                            }
                        });
                };

                ctx.show_viewport_immediate(viewport_id, viewport_builder, viewport_cb);
            }
        });
    }
}
