use eframe::egui::{self, Id, Pos2, Vec2};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyApp {
    #[serde(skip)]
    show_a: bool,
    #[serde(skip)]
    show_b: bool,

    // one-shot runtime flags (not persisted)
    #[serde(skip)]
    place_a_on_next_frame: bool,
    #[serde(skip)]
    place_b_on_next_frame: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_a: false,
            show_b: false,
            place_a_on_next_frame: false,
            place_b_on_next_frame: false,
        }
    }
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // no restore from storage
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // no-op: app state not persisted
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // A controls
                if self.show_a {
                    if ui.button("Close A").clicked() {
                        self.show_a = false;
                    }
                } else if ui.button("Open A").clicked() {
                    self.show_a = true;
                    self.place_a_on_next_frame = true;
                }

                // B controls
                if self.show_b {
                    if ui.button("Close B").clicked() {
                        self.show_b = false;
                    }
                } else if ui.button("Open B").clicked() {
                    self.show_b = true;
                    self.place_b_on_next_frame = true;
                }

                ui.separator();

                if ui.button("Clear egui memory").clicked() {
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(win) = web_sys::window() {
                            if let Ok(Some(storage)) = win.local_storage() {
                                let _ = storage.remove_item("egui_memory_ron");
                            }
                        }
                    }

                    // Clear in-memory egui state for current session
                    ctx.memory_mut(|m| *m = Default::default());
                    ctx.request_repaint();
                }
            });
        });

        let avail = ctx.available_rect();
        let center = avail.center();

        // ---- Window A ----
        let a_size = Vec2::new(260.0, 120.0);
        let a_center_pos = Pos2::new(center.x - a_size.x * 0.5, center.y - a_size.y * 0.5);

        let a_rect = if self.show_a {
            let mut a = egui::Window::new("A")
                .id(Id::new("window_a"))
                .collapsible(false)
                .movable(true)
                .resizable(true)
                .default_size(a_size)
                .open(&mut self.show_a);

            if self.place_a_on_next_frame {
                a = a.current_pos(a_center_pos);
            }

            let rect = a
                .show(ctx, |ui| {
                    ui.label("Window A");
                })
                .map(|ir| ir.response.rect);

            self.place_a_on_next_frame = false;
            rect
        } else {
            self.place_a_on_next_frame = false;
            None
        };

        // ---- Window B ----
        if self.show_b {
            let b_size = Vec2::new(400.0, 180.0);

            let mut b = egui::Window::new("B")
                .id(Id::new("window_b"))
                .collapsible(false)
                .movable(true)
                .resizable(true)
                .default_size(b_size)
                .min_width(400.0)
                .open(&mut self.show_b);

            if self.place_b_on_next_frame {
                if self.show_a {
                    if let Some(r) = a_rect {
                        let gap = 12.0;
                        b = b.current_pos(Pos2::new(r.right() + gap, r.top()));
                    } else {
                        b = b.current_pos(Pos2::new(
                            center.x - b_size.x * 0.5,
                            center.y - b_size.y * 0.5,
                        ));
                    }
                } else {
                    b = b.current_pos(Pos2::new(
                        center.x - b_size.x * 0.5,
                        center.y - b_size.y * 0.5,
                    ));
                }
            }

            b.show(ctx, |ui| {
                ui.label("Window B");
            });

            self.place_b_on_next_frame = false;
        } else {
            self.place_b_on_next_frame = false;
        }
    }
}
