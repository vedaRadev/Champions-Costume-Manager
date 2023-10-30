use std::{ fs, path::Path };
use eframe::egui;
use rfd::FileDialog;

mod costume;
use costume::CostumeSave;

const DEFAULT_COSTUME_DIR: &str = "/mnt/c/Program Files (x86)/Steam/steamapps/common/Champions Online/Champions Online/Live/screenshots";

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

struct SelectedCostume {
    file_path: String,
    save_data: CostumeSave,
    character_name: String,
    account_name: String,
}

struct ChampionsCostumeManager {
    costumes_dir: String,
    costumes: Option<Vec<String>>,
    selected_costume: Option<SelectedCostume>,
}

impl ChampionsCostumeManager {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            costumes_dir: String::from(""), // TODO make this an Option
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
                if let Some(selected_costume) = &mut self.selected_costume {
                    ui.add(
                        egui::Image::new(format!("file://{}", selected_costume.file_path))
                            .rounding(10.0)
                            .maintain_aspect_ratio(true)
                            .shrink_to_fit()
                    );

                    ui.separator();

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.label("Account Name:");
                        ui.add(egui::TextEdit::singleline(&mut selected_costume.account_name));
                    });

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.label("Character Name:");
                        ui.add(egui::TextEdit::singleline(&mut selected_costume.character_name));
                    });

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP).with_main_align(egui::Align::Center), |ui| {
                        if ui.button("Save").clicked() {
                            selected_costume.save_data.set_account_name(&selected_costume.account_name);
                            selected_costume.save_data.set_character_name(&selected_costume.character_name);
                            selected_costume.save_data.write_to_file(Path::new(&selected_costume.file_path)).expect("Failed to write costume data");
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
                            for costume_path in costumes {
                                let costume_image = egui::Image::new(format!("file://{}", costume_path))
                                    .rounding(10.0)
                                    .fit_to_original_size(0.5)
                                    .maintain_aspect_ratio(true);

                                if ui.add(egui::ImageButton::new(costume_image)).clicked() {
                                    // TODO recover from error
                                    let mut save_data = CostumeSave::from_file(Path::new(costume_path)).expect("Failed to parse costume data");

                                    self.selected_costume = Some(SelectedCostume {
                                        file_path: costume_path.into(),
                                        character_name: save_data.get_character_name(),
                                        account_name: save_data.get_account_name(),
                                        save_data,
                                    });
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
                            if let Some(costumes_dir) = FileDialog::new()
                                .set_directory(DEFAULT_COSTUME_DIR)
                                .pick_folder() {

                                self.costumes_dir = costumes_dir.to_str().unwrap().to_string();
                                self.costumes = Some(get_saved_costumes(&costumes_dir));
                            }
                        }
                    }
                );
            }
        });
    }
}
