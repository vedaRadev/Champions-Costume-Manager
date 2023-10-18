use std::{ fs, path::{ Path, PathBuf } };
use eframe::egui;

const COSTUME_SAVES_PATH: &str = "/mnt/c/Program Files (x86)/Steam/steamapps/common/Champions Online/Champions Online/Live/screenshots";

// TODO
// * Only return files that are valid costume saves.
// * Ensure this is performant when the dir contains many files.
// * Actual error handling, maybe return a result containing the vec.
fn get_saved_costumes(saves_dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(saves_dir).unwrap().map(|p| p.unwrap().path()).collect()
}

fn main() -> Result<(), eframe::Error> {
    let costumes = get_saved_costumes(Path::new(COSTUME_SAVES_PATH));
    println!("{:?}", costumes);

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Test App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(ChampionsCostumeManager::new(costumes))
        })
    )
}

struct ChampionsCostumeManager {
    costume_uris: Vec<String>,
}

impl ChampionsCostumeManager {
    fn new(costume_paths: Vec<PathBuf>) -> Self {
        ChampionsCostumeManager {
            costume_uris: costume_paths.into_iter()
                .map(|p| format!("file://{}", p.to_str().unwrap()))
                .collect()
        }
    }
}

impl eframe::App for ChampionsCostumeManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for uri in &self.costume_uris {
                ui.add(egui::Image::new(uri));
            }
        });
    }
}
