use std::{ fs, path::Path };
use eframe::egui;
use rfd::FileDialog;

const DEFAULT_COSTUME_DIR: &str = "/mnt/c/Program Files (x86)/Steam/steamapps/common/Champions Online/Champions Online/Live/screenshots";

// TODO
// * Only return files that are valid costume saves.
// * Ensure this is performant when the dir contains many files.
// * Actual error handling, maybe return a result containing the vec.
fn get_saved_costumes(saves_dir: &Path) -> Vec<String> {
    fs::read_dir(saves_dir).unwrap()
        .map(|p| format!("file://{}", String::from(p.unwrap().path().to_str().unwrap())))
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
    costumes: Option<Vec<String>>,
    selected_costume: Option<String>,
}

impl ChampionsCostumeManager {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            selected_costume: None,
            costumes: None,
        }
    }
}

// TODO refactor to pull the image details and costume selection out into their own components
impl eframe::App for ChampionsCostumeManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // IMAGE DETAILS PANEL
        egui::SidePanel::right("costume_preview_panel")
            .resizable(false)
            .min_width(250.0)
            .show(ctx, |ui| {
                if let Some(selected_costume) = &self.selected_costume {
                    ui.add(
                        egui::Image::new(selected_costume)
                            .rounding(10.0)
                            .maintain_aspect_ratio(true)
                            .shrink_to_fit()
                    );

                    ui.separator();

                    ui.label("Placeholder Name");
                    ui.label("Placeholder Character");
                    ui.label("Placeholder Owner");

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP).with_main_align(egui::Align::Center), |ui| {
                        if ui.button("Edit").clicked() {
                            // TODO
                        }

                        if ui.button("Delete").clicked() {
                            // TODO
                        }
                    });
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label("Select an image to edit details")
                    });
                }
            });

        // COSTUME SELECTION
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(costumes) = &self.costumes {
                // DISPLAY COSTUMES
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for costume in costumes {
                                let costume_image = egui::Image::new(costume)
                                    .rounding(10.0)
                                    .fit_to_original_size(0.5)
                                    .maintain_aspect_ratio(true);

                                if ui.add(egui::ImageButton::new(costume_image)).clicked() {
                                    self.selected_costume = Some(costume.to_owned());
                                }
                            }
                        });
                });
            } else {
                // PROMPT FOR DIRECTORY SELECT
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        let button = egui::Button::new("Select costumes directory");
                        if ui.add(button).clicked() {
                            if let Some(costumes_dir_path) = FileDialog::new()
                                .set_directory(DEFAULT_COSTUME_DIR)
                                .pick_folder() {
                                self.costumes = Some(get_saved_costumes(&costumes_dir_path));
                            }
                        }
                    }
                );
            }
        });
    }
}
