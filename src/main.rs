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
    costume_specification: String,
}

// TODO implement state machine for central panel state. Could also maybe do this for popups too.
struct ChampionsCostumeManager {
    costumes_dir: String,
    costumes: Option<Vec<String>>,
    selected_costume: Option<SelectedCostume>,
    show_delete_confirmation: bool,
    show_edit_costume_specification: bool,
}

impl ChampionsCostumeManager {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            costumes_dir: String::from(""), // TODO make this an Option
            selected_costume: None,
            costumes: None,
            show_delete_confirmation: false,
            show_edit_costume_specification: false,
        }
    }
}

// TODO refactor to pull the image details and costume selection out into their own components
impl eframe::App for ChampionsCostumeManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // COSTUME DELETION CONFIRMATION POPUP
        // TODO: disable interactivity with other controls until popup is dealt with_layout
        // NOTE: this should also really be checking that a costume is selected, not that we should
        // be able to get into this state without one
        if self.show_delete_confirmation {
            let sr_size = ctx.screen_rect().size();

            egui::Window::new("Delete Save?")
                .collapsible(false)
                .movable(false)
                .resizable(false)
                .title_bar(true)
                .pivot(egui::Align2::CENTER_CENTER)
                .fixed_pos([sr_size.x / 2.0, sr_size.y / 2.0])
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        if ui.button("Confirm").clicked() {
                            let file_path = &self.selected_costume.as_ref().expect("No costume selected").file_path;
                            fs::remove_file(file_path).expect("Failed to delete file");

                            self.costumes = Some(get_saved_costumes(Path::new(&self.costumes_dir)));
                            self.selected_costume = None;
                            self.show_delete_confirmation = false;
                        }

                        if ui.button("Cancel").clicked() {
                            self.show_delete_confirmation = false;
                        }
                    });
                });
        }

        // COSTUME SPECIFICATION POPUP
        // TODO: disable interactivity with other controls until popup is dealt with_layout
        // NOTE: this should also really be checking that a costume is selected, not that we should
        // be able to get into this state without one
        if self.show_edit_costume_specification {
            let sr_size = ctx.screen_rect().size();
            let win_size = egui::vec2(sr_size.x - 50.0, sr_size.y - 50.0);

            egui::Window::new("Costume Specification")
                .collapsible(false)
                .movable(false)
                .resizable(false)
                .constrain(true)
                .title_bar(true)
                .fixed_size(win_size)
                .vscroll(true)
                .pivot(egui::Align2::CENTER_CENTER)
                .default_pos([sr_size.x / 2.0, sr_size.y / 2.0])
                .show(ctx, |ui| {
                    let selected_costume = &mut self.selected_costume.as_mut().unwrap();

                    // FIXME: Putting the buttons above the editor for now because I can't figure
                    // out how to not make the editor freak out in the other orientation
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        if ui.button("Save Specification").clicked() {
                            selected_costume.save_data.set_costume_specification(&selected_costume.costume_specification);
                            self.show_edit_costume_specification = false;
                        }

                        if ui.button("Cancel").clicked() {
                            self.show_edit_costume_specification = false;
                        }
                    });

                    egui::ScrollArea::vertical()
                        .auto_shrink([true, true])
                        .show(ui, |ui| {
                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::multiline(&mut selected_costume.costume_specification)
                                    .code_editor()
                            );
                        });
                });
        }

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

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Preview:");
                        ui.label(costume::to_simulated_save_name(
                            Path::new(&selected_costume.file_path),
                            &selected_costume.account_name,
                            &selected_costume.character_name
                        ));
                    });

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP).with_main_align(egui::Align::Center), |ui| {
                        if ui.button("Save").clicked() {
                            selected_costume.save_data.set_account_name(&selected_costume.account_name);
                            selected_costume.save_data.set_character_name(&selected_costume.character_name);
                            selected_costume.save_data.write_to_file(Path::new(&selected_costume.file_path)).expect("Failed to write costume data");
                        }

                        if ui.button("Delete").clicked() {
                            self.show_delete_confirmation = true;
                        }

                        if ui.button("Show Costume Specification").clicked() {
                            self.show_edit_costume_specification = true;
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
                if !costumes.is_empty() {
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
                                        let mut save_data = CostumeSave::from_file(Path::new(costume_path)).expect("Failed to parse costume data");

                                        self.selected_costume = Some(SelectedCostume {
                                            file_path: costume_path.into(),
                                            character_name: save_data.get_character_name(),
                                            account_name: save_data.get_account_name(),
                                            costume_specification: save_data.get_costume_specification(),
                                            save_data,
                                        });
                                    }
                                }
                            });
                        });
                } else {
                    // INFORM NO SAVES
                    ui.with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| ui.label("No costumes found")
                    );
                }
            } else {
                // PROMPT FOR DIRECTORY SELECT
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        if ui.button("Select costumes directory").clicked() {
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
