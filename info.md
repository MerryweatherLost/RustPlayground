#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
mod school;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Job Creation Tool",
        options,
        Box::new(|_cc| Box::new(CareerData::default())),
    )
}

struct CareerData {
    job_name: String,
    company_name: String,
}

impl Default for CareerData {
    fn default() -> Self {
        Self {
            job_name: "".to_owned(),
            company_name: "".to_owned(),
        }
    }
}

impl eframe::App for CareerData {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.set_decorations(true);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Job Creation Tool");
            ui.add(egui::Separator::default().spacing(30 as f32));
            ui.vertical(|ui| {
                // Job Title & Associated Popup
                ui.label("Enter Job Title");
                ui.text_edit_singleline(&mut self.job_name);
                ui.add_space(15 as f32);
                ui.label("Enter Company Name");
                ui.text_edit_singleline(&mut self.company_name)
            });
            ui.label(format!(
                "Job Name: {} :: Company Name: {}",
                self.job_name, self.company_name
            ))
        });
    }
}

// let popup_job_title_id = ui.make_persistent_id("POPUP.JOB.TITLE");
// if ui.text_edit_singleline(&mut self.job_name).hovered() {
//     ui.memory().toggle_popup(popup_job_title_id)
// }
// egui::popup::popup_below_widget(ui, popup_job_title_id, &job_name_label, |ui| {
//     ui.label("Enter the name of the job here.");
// });
