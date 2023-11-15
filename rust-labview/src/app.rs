use egui::{Pos2, Window, Sense, epaint::{Shadow, CubicBezierShape, self}, Style, Visuals, Color32, pos2};
use function_widget::{FunctionWidget, Runnable, ParamTypes, FunctionConfig};

struct LinkVertex {
    function_name: String,
    entry_name: String,
}

struct Link {
    start: LinkVertex,
    end: LinkVertex,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    rects: Vec<FunctionConfig>,
    #[serde(skip)] 
    links: Vec<Link>,
    // curve_starting_pos: Option<Pos2>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            rects: vec![ 
                FunctionConfig::default(),
                FunctionConfig::default_with_pos(Pos2 {x: 180.0, y: 40.0}, "Function #1".to_owned()),
            ],
            links: vec![
                Link { 
                    start: LinkVertex { function_name: "Function #0".to_owned(), entry_name: "Output1".to_owned() },
                    end: LinkVertex { function_name: "Function #1".to_owned(), entry_name: "Input1".to_owned() }
                }
            ]
            // curve_starting_pos: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // cc.egui_ctx.set_debug_on_hover(true);

        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        self.rects.retain(|ele| {ele.is_open});

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::left("Toolbox").show(ctx, |ui| {
            let icon = 
                egui::Button::image_and_text(egui::include_image!("../assets/function-icon.png"), "Add function")
                .rounding(5.0);
            let icon_response = ui.add(icon);
            if icon_response.clicked() {
                self.rects.push(FunctionConfig::default_with_pos(Pos2 { x: 0.0, y: 0.0 }, format!("Function #{}", self.rects.len())));
            }
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            powered_by_egui_and_eframe(ui)
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for ele in self.rects.iter_mut() {
                    ui.add(&mut FunctionWidget::new(ele));
                }
            });

            let stroke = ui.visuals().widgets.hovered.bg_stroke;

            for link in self.links.iter() {
                let start_point = self.rects.iter()
                    .find(|p| p.runnable.name == link.start.function_name)
                    .expect(format!("Non-connected link is found: function '{}' is not found", link.start.function_name).as_str())
                    .runnable.outputs.get(&link.start.entry_name)
                    .expect(format!("Non-connected link is found: output '{}' is not found in '{}'", link.start.entry_name, link.start.function_name).as_str())
                    .pos;
                let end_point = self.rects.iter()
                    .find(|p| p.runnable.name == link.end.function_name)
                    .expect(format!("Non-connected link is found: function '{}' is not found", link.end.function_name).as_str())
                    .runnable.inputs.get(&link.end.entry_name)
                    .expect(format!("Non-connected link is found: input '{}' is not found in '{}'", link.end.entry_name, link.end.function_name).as_str())
                    .pos;

                let second_point = Pos2 { x: (start_point.x + end_point.x)/2.0, y: start_point.y};
                let third_point = Pos2 { x: (start_point.x + end_point.x)/2.0, y: end_point.y };

                let points: [Pos2; 4] = [start_point,second_point, third_point, end_point];
                let curve = CubicBezierShape::from_points_stroke(points, false, Default::default(), stroke);
                
                ui.painter().add(curve);
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
