fn main() -> eframe::Result {
    eframe::run_native(
        "MakeMyTieR",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        
    }
}