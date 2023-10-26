use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path
};
use eframe::egui;
use rfd::FileDialog;

const DEFAULT_COSTUME_DIR: &str = "/mnt/c/Program Files (x86)/Steam/steamapps/common/Champions Online/Champions Online/Live/screenshots";

struct Record {
    data_sets: Vec<DataSet>
}

struct DataSet {
    tag: DataSetTag,
    data: Vec<u8>,
}

struct DataSetTag {
    tag_marker: u8,
    record_number: u8,
    data_set_number: u8,
    data_field_octet_count: u16, // i.e. data field length in bytes
}

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
                        egui::Image::new(format!("file://{}", selected_costume))
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
                                let costume_image = egui::Image::new(format!("file://{}", costume))
                                    .rounding(10.0)
                                    .fit_to_original_size(0.5)
                                    .maintain_aspect_ratio(true);

                                if ui.add(egui::ImageButton::new(costume_image)).clicked() {
                                    self.selected_costume = Some(costume.to_owned());

                                    // GRAB RELEVANT METADATA
                                    let file = fs::File::open(costume).expect("failed to open file");
                                    let mut reader = BufReader::new(file);

                                    // Skip to app13 data segment
                                    let _ = reader.seek_relative(2); // skip SOI
                                    loop {
                                        let mut segment = [0u8; 2];
                                        let mut length = [0u8; 2];
                                        let _ = reader.read_exact(&mut segment);
                                        let _ = reader.read_exact(&mut length);
                                        let length = unsafe { std::mem::transmute::<[u8; 2], u16>(length).to_be() } as usize;

                                        match segment {
                                            [0xFF, 0xED] => {
                                                println!("APP13 Segment Length: {}", length);

                                                let mut segment_id: Vec<u8> = vec![];
                                                let _ = reader.read_until(0x00, &mut segment_id);
                                                let segment_id = std::str::from_utf8(&segment_id).unwrap();
                                                println!("APP13 Segment ID: {}", segment_id);

                                                let mut segment_signature = [0u8; 4];
                                                let _ = reader.read_exact(&mut segment_signature);
                                                let segment_signature = std::str::from_utf8(&segment_signature).unwrap();
                                                println!("APP13 Segment signature: {}", segment_signature);

                                                // 0x0404 is IPTC-NAA record, contains "File Info..." information
                                                let mut resource_id = [0u8; 2];
                                                let _ = reader.read_exact(&mut resource_id);
                                                println!("Resource ID: {:04X?}", resource_id);

                                                // Name: costume saves don't use these so it is
                                                // 0x00 0x00 for null name. We'll just skip them.
                                                let _ = reader.seek_relative(2);

                                                let mut resource_length = [0u8; 4];
                                                let _ = reader.read_exact(&mut resource_length);
                                                let resource_length = unsafe { std::mem::transmute::<[u8; 4], u32>(resource_length).to_be() } as usize;
                                                println!("Resource length: {}", resource_length);

                                                /*
                                                * SKIP READING IN THE APP13 RESOURCE DATA
                                                * INSTEAD WE WILL PARSE RECORDS INDIVIDUALLY
                                                */
                                                // let mut resource_data = vec![0u8; resource_length];
                                                // let _ = reader.read_exact(&mut resource_data);
                                                // println!("{}", unsafe { std::str::from_utf8_unchecked(&resource_data) });

                                                let current_position = reader.stream_position().expect("Failed to get position in file buffer");
                                                let end_of_segment = current_position + resource_length as u64;
                                                while reader.stream_position().expect("Failed to get position in buffer") < end_of_segment {
                                                    println!();
                                                    println!("RECORD");

                                                    let mut tag_marker = [0u8; 1];
                                                    let _ = reader.read_exact(&mut tag_marker);
                                                    println!("Tag marker: {:02X?}", tag_marker);

                                                    let mut record_number = [0u8; 1];
                                                    let _ = reader.read_exact(&mut record_number);
                                                    println!("Record number: {:02X?}", record_number);

                                                    let mut data_set_number = [0u8; 1];
                                                    let _ = reader.read_exact(&mut data_set_number);
                                                    println!("Data set number: {:02X?}", data_set_number);

                                                    let mut data_field_length = [0u8; 2];
                                                    let _ = reader.read_exact(&mut data_field_length);
                                                    let data_field_length = unsafe { std::mem::transmute::<[u8; 2], u16>(data_field_length).to_be() } as usize;
                                                    println!("Data field length: {}", data_field_length);

                                                    let mut data = vec![0u8; data_field_length];
                                                    let _ = reader.read_exact(&mut data);
                                                    let data = unsafe { std::str::from_utf8_unchecked(&data) };
                                                    println!("Data: {}", data);
                                                }

                                                break;
                                            },
                                            _ => { 
                                                let _ = reader.seek_relative(length as i64 - 2); 
                                            }
                                        }
                                    }
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
