use std::{ fs, path::Path };
use eframe::egui;
// use rfd::FileDialog;

const COSTUME_SAVES_PATH: &str = "/mnt/c/Program Files (x86)/Steam/steamapps/common/Champions Online/Champions Online/Live/screenshots";

// TODO
// * Only return files that are valid costume saves.
// * Ensure this is performant when the dir contains many files.
// * Actual error handling, maybe return a result containing the vec.
fn get_saved_costumes(saves_dir: &Path) -> Vec<String> {
    fs::read_dir(saves_dir).unwrap()
        .map(|p| String::from(p.unwrap().path().to_str().unwrap()))
        .collect()
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Champions Costume Manager",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(ChampionsCostumeManager::new(cc))
        })
    )
}

struct ChampionsCostumeManager {
    costume_saves: Vec<String>
}

impl ChampionsCostumeManager {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            costume_saves: get_saved_costumes(Path::new(COSTUME_SAVES_PATH))
        }
    }
}

impl eframe::App for ChampionsCostumeManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // IMAGE DETAILS PANEL
        egui::SidePanel::right("costume_preview_panel")
            .resizable(false)
            .min_width(250.0)
            .show(ctx, |ui| {
                ui.add(
                    egui::Image::new(format!("file://{}", &self.costume_saves[0]))
                        .rounding(10.0)
                        .maintain_aspect_ratio(true)
                        .shrink_to_fit()
                );

                egui::TopBottomPanel::top("costume_details_panel").show_inside(ui, |ui| {
                    ui.label("Placeholder Name");
                    ui.label("Placeholder Character");
                    ui.label("Placeholder Owner");
                });
            });

        // COSTUME SELECTION
        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_width = ui.available_width();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_width(panel_width);

                ui.horizontal_wrapped(|ui| {
                    for costume in &self.costume_saves {
                        ui.add(
                            egui::Image::new(format!("file://{}", costume))
                                .rounding(10.0)
                                .fit_to_original_size(0.5)
                                .maintain_aspect_ratio(true)
                        );
                    }
                });
            });
        });
    }
}
