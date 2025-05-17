use eframe::{egui, App, Frame};
use egui::{
    Align, CentralPanel, Color32, Context, CornerRadius, FontId, Layout, Margin, ScrollArea,
};
use utils::{redirect_minecraft_to_a_port, remove_minecraft_redirect, scan_ports};

pub mod error;
pub mod utils;

// Define the structure for an entry
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Entry {
    id: u32, // Unique ID for each entry
    title: String,
    port: u16,
    redirected_port: Option<u16>,
}

impl From<u16> for Entry {
    fn from(value: u16) -> Self {
        Entry {
            id: value as u32,
            title: "Minecraft game".to_string(),
            port: value,
            redirected_port: None,
        }
    }
}

// Main application struct
struct MyApp {
    available_entries: Vec<Entry>,
    enabled_entries: Vec<Entry>,
    port_input: String,
    selected_available_id: Option<u32>,
    selected_enabled_id: Option<u32>,
    error_string: String,
    local_ip: String,
    public_ip: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let initial_entries = scan_ports()
            .unwrap_or_default()
            .into_iter()
            .map(Entry::from)
            .collect();
        Self {
            available_entries: initial_entries,
            enabled_entries: Vec::new(),
            port_input: "8080".to_string(),
            selected_available_id: None,
            selected_enabled_id: None,
            error_string: String::new(),
            local_ip: utils::get_local_ip()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "N/A".to_string()),
            public_ip: utils::get_public_address()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "N/A".to_string()),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // General styling
            let heading_font = FontId::proportional(18.0);
            let field_min_width = 180.0; // Min width for IP display "fields"

            // --- Network Information Section ---
            ui.heading("Network Information");
            ui.add_space(5.0);

            // Local IP display
            ui.horizontal(|ui| {
                ui.label("Local IP:");
                ui.add_space(4.0);
                egui::Frame::default()
                    .inner_margin(Margin::symmetric(6, 4)) // Corrected Margin arguments to i8
                    .corner_radius(CornerRadius::same(4)) // Corrected to CornerRadius and u8 argument
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .show(ui, |ui| {
                        ui.set_min_width(field_min_width);
                        ui.label(&self.local_ip);
                    });
                if ui.button("📋").on_hover_text("Copy Local IP").clicked() {
                    ctx.copy_text(self.local_ip.clone());
                }
            });
            ui.add_space(5.0);

            // Public IP display
            ui.horizontal(|ui| {
                ui.label("Public IP:");
                ui.add_space(4.0);
                egui::Frame::default()
                    .inner_margin(Margin::symmetric(6, 4)) // Corrected Margin arguments to i8
                    .corner_radius(CornerRadius::same(4)) // Corrected to CornerRadius and u8 argument
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .show(ui, |ui| {
                        ui.set_min_width(field_min_width);
                        ui.label(&self.public_ip);
                    });
                if ui.button("📋").on_hover_text("Copy Public IP").clicked() {
                    ctx.copy_text(self.public_ip.clone());
                }
            });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(10.0);

            // --- Available Entries Section ---
            ui.label(egui::RichText::new("Available Entries").font(heading_font.clone()));
            ScrollArea::vertical()
                .id_salt("available_entries_scroll")
                .max_height(180.0)
                .show(ui, |ui| {
                    for entry in &self.available_entries {
                        let label = format!("{} ({})", entry.title, entry.port);
                        if ui
                            .selectable_label(self.selected_available_id == Some(entry.id), label)
                            .clicked()
                        {
                            self.selected_available_id =
                                if self.selected_available_id == Some(entry.id) {
                                    None
                                } else {
                                    Some(entry.id)
                                };
                            if self.selected_available_id.is_some() {
                                self.selected_enabled_id = None;
                            }
                        }
                    }
                });

            ui.add_space(10.0);

            // --- Port Input Section ---
            ui.horizontal(|ui| {
                ui.label("Port:");
                ui.add_sized(
                    [100.0, 20.0],
                    egui::TextEdit::singleline(&mut self.port_input).id_source("port_input_field"),
                );
            });

            ui.add_space(10.0);

            // --- Enabled Entries Section ---
            ui.label(egui::RichText::new("Enabled Entries").font(heading_font.clone()));
            ScrollArea::vertical()
                .id_salt("enabled_entries_scroll")
                .max_height(180.0)
                .show(ui, |ui| {
                    for entry in &self.enabled_entries {
                        let label = format!("{} ({})", entry.title, entry.port);
                        if ui
                            .selectable_label(self.selected_enabled_id == Some(entry.id), label)
                            .clicked()
                        {
                            self.selected_enabled_id = if self.selected_enabled_id == Some(entry.id)
                            {
                                None
                            } else {
                                Some(entry.id)
                            };
                            if self.selected_enabled_id.is_some() {
                                self.selected_available_id = None;
                            }
                        }
                    }
                });

            ui.add_space(15.0);

            // --- Enable/Disable Button Section ---
            let (button_text, button_is_enabled) = if self.selected_available_id.is_some() {
                ("Enable Selected", true)
            } else if self.selected_enabled_id.is_some() {
                ("Disable Selected", true)
            } else {
                ("Select an item", false)
            };

            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                if ui
                    .add_enabled(
                        button_is_enabled,
                        egui::Button::new(button_text).min_size(egui::vec2(180.0, 30.0)),
                    )
                    .on_hover_text_at_pointer("Click to move the selected item between lists")
                    .clicked()
                {
                    if let Some(id_to_enable) = self.selected_available_id.take() {
                        if let Some(index) = self
                            .available_entries
                            .iter()
                            .position(|e| e.id == id_to_enable)
                        {
                            let mut entry_to_enable = self.available_entries.remove(index);
                            let Ok(external_port) = self.port_input.parse::<u16>() else {
                                self.error_string = "Invalid port detected!".to_string();
                                self.available_entries.insert(index, entry_to_enable);
                                return;
                            };
                            if let Err(e) = redirect_minecraft_to_a_port(
                                entry_to_enable.port,
                                external_port,
                                3600,
                            ) {
                                self.error_string = e.to_string();
                                self.available_entries.insert(index, entry_to_enable);
                                return;
                            }
                            entry_to_enable.redirected_port = Some(external_port);
                            self.port_input = String::new();
                            self.enabled_entries.push(entry_to_enable);
                            self.enabled_entries.sort_by_key(|e| e.id);
                        }
                    } else if let Some(id_to_disable) = self.selected_enabled_id.take() {
                        if let Some(index) = self
                            .enabled_entries
                            .iter()
                            .position(|e| e.id == id_to_disable)
                        {
                            let mut entry_to_disable = self.enabled_entries.remove(index);
                            if let Err(e) =
                                remove_minecraft_redirect(entry_to_disable.redirected_port.unwrap())
                            {
                                self.error_string = e.to_string();
                                self.enabled_entries.insert(index, entry_to_disable);
                                return;
                            }
                            entry_to_disable.redirected_port = None;
                            self.available_entries.push(entry_to_disable);
                            self.available_entries.sort_by_key(|e| e.id);
                        }
                    }
                }
            });
            ui.label(
                egui::RichText::new(self.error_string.clone())
                    .font(heading_font.clone())
                    .color(Color32::RED),
            );
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Open 2 Internet",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
