use eframe::{egui, App, Frame};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

use crate::core::types::{PhotoFile, OrganizationResult, OrganizeSettings, AppConfig};
use crate::core::scanner::scan_directory;
use crate::core::organizer::{validate_directories, create_target_structure, organize_files};

#[derive(Debug, PartialEq, Clone)]
pub enum AppTab {
    Organize,
    Settings,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProcessState {
    Idle,
    Analyzing,
    AnalyzeComplete(Vec<PhotoFile>), 
    Organizing,
}

pub struct PhotoOrganizerApp {
    // Navigation
    current_tab: AppTab,
    is_welcome_screen: bool,

    // Configuration
    source_dir: Option<PathBuf>,
    target_dir: Option<PathBuf>,
    
    // Process State
    state: ProcessState,
    scan_receiver: Option<Receiver<ScanUpdate>>,
    organize_receiver: Option<Receiver<OrganizeUpdate>>,
    
    // Data
    found_files: Vec<PhotoFile>,
    organization_result: Option<OrganizationResult>,
    show_support_modal: bool,
    
    // UI Feedback
    status_message: String,
    progress: f32,
    error_message: Option<String>,
    show_warning_modal: bool,
}

pub enum ScanUpdate {
    Complete(Vec<PhotoFile>), 
    Error(String),
}

pub enum OrganizeUpdate {
    Complete(OrganizationResult),
}

impl Default for PhotoOrganizerApp {
    fn default() -> Self {
        Self {
            current_tab: AppTab::Organize,
            is_welcome_screen: true,
            source_dir: None,
            target_dir: None,
            state: ProcessState::Idle,
            scan_receiver: None,
            organize_receiver: None,
            found_files: Vec::new(),
            organization_result: None,
            show_support_modal: false,
            status_message: "Ready".to_string(),
            progress: 0.0,
            error_message: None,
            show_warning_modal: false,
        }
    }
}

impl PhotoOrganizerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load();
        
        let mut app = Self {
            current_tab: AppTab::Organize,
            is_welcome_screen: true, 
            source_dir: config.last_source_dir.map(PathBuf::from),
            target_dir: config.last_target_dir.map(PathBuf::from),
            state: ProcessState::Idle,
            scan_receiver: None,
            organize_receiver: None,
            found_files: Vec::new(),
            organization_result: None,
            show_support_modal: false,
            status_message: "Ready".to_string(),
            progress: 0.0,
            error_message: None,
            show_warning_modal: false,
        };
        
        app.configure_style(&cc.egui_ctx);
        
        // Smart Startup
        if let Some(target) = &app.target_dir {
             if target.exists() {
                 app.is_welcome_screen = false;
             } else {
                 app.is_welcome_screen = true;
             }
        } else {
             app.is_welcome_screen = true;
        }
        
        app
    }
    
    fn configure_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        // Theme Colors
        let _primary_color = egui::Color32::from_rgb(39, 74, 102); // #274A66
        let _primary_light = egui::Color32::from_rgb(59, 94, 122); 
        
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_gray(248); // Light bg
        style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::WHITE;
        
        // Rounded corners for modern feel
        let _corner_radius = egui::CornerRadius::same(6);
        style.visuals.widgets.active.bg_stroke.color = egui::Color32::from_rgb(39, 74, 102);
        style.visuals.widgets.inactive.bg_stroke.color = egui::Color32::from_gray(200);
        
        // Fonts
        style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(14.0, egui::FontFamily::Proportional));
        style.text_styles.insert(egui::TextStyle::Button, egui::FontId::new(14.0, egui::FontFamily::Proportional));
        style.text_styles.insert(egui::TextStyle::Heading, egui::FontId::new(20.0, egui::FontFamily::Proportional));

        // Spacing
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        
        ctx.set_style(style);
    }
    
    fn save_config(&self) {
        let config = AppConfig {
            last_source_dir: self.source_dir.as_ref().map(|p| p.to_string_lossy().to_string()),
            last_target_dir: self.target_dir.as_ref().map(|p| p.to_string_lossy().to_string()),
            window_width: None,
            window_height: None,
        };
        config.save();
    }

    // --- Background Operations ---


    fn start_analysis(&mut self) {
        self.error_message = None;
        self.found_files.clear();
        
        let source = self.source_dir.clone().unwrap();
        let target = self.target_dir.clone().unwrap();
        
        if let Err(e) = validate_directories(&source, &target) {
            self.error_message = Some(e.to_string());
            return;
        }
        
        let (tx, rx) = channel();
        self.scan_receiver = Some(rx);
        self.state = ProcessState::Analyzing;
        self.status_message = "Scanning files...".to_string();
        
        thread::spawn(move || {
            if let Err(e) = create_target_structure(&target) {
                let _ = tx.send(ScanUpdate::Error(format!("Target prep failed: {}", e)));
                return;
            }
            match scan_directory(&source) {
                Ok(files) => { let _ = tx.send(ScanUpdate::Complete(files)); },
                Err(e) => { let _ = tx.send(ScanUpdate::Error(e.to_string())); }
            }
        });
    }

    fn start_organizing(&mut self) {
        let (tx, rx) = channel();
        self.organize_receiver = Some(rx);
        self.state = ProcessState::Organizing;
        self.status_message = "Organizing...".to_string();
        self.progress = 0.0;
        
        let files = self.found_files.clone();
        let settings = OrganizeSettings {
            target_dir: self.target_dir.clone().unwrap(),
            dry_run: false,
        };
        
        thread::spawn(move || {
            let res = organize_files(files, &settings);
            let _ = tx.send(OrganizeUpdate::Complete(res));
        });
    }
    
    fn poll_updates(&mut self) {
        // Scan polling
        if let Some(rx) = &self.scan_receiver {
            if let Ok(update) = rx.try_recv() {
                match update {
                    ScanUpdate::Complete(files) => {
                        self.found_files = files;
                        self.state = ProcessState::AnalyzeComplete(self.found_files.clone());
                        self.scan_receiver = None; 
                    },
                    ScanUpdate::Error(e) => {
                        self.error_message = Some(e);
                        self.state = ProcessState::Idle;
                        self.scan_receiver = None;
                    }
                }
            }
        }
        
        // Organize polling
        if let Some(rx) = &self.organize_receiver {
             if let Ok(update) = rx.try_recv() {
                 match update {
                     OrganizeUpdate::Complete(res) => {
                         self.organization_result = Some(res);
                         self.state = ProcessState::Idle;
                         self.organize_receiver = None;
                     },
                 }
             }
        }
    }


    // --- Views ---

    fn render_sidebar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.heading("Photo & Video\nOrganizer");
        ui.add_space(30.0);
        
        let current = self.current_tab.clone();
        
        let tab_btn = |ui: &mut egui::Ui, text: &str, _tab: AppTab, is_selected: bool| {
            let btn = egui::Button::new(text)
                .min_size(egui::vec2(ui.available_width(), 40.0));
            ui.add_enabled(!is_selected, btn).clicked()
        };

        if tab_btn(ui, "📥  Organize", AppTab::Organize, current == AppTab::Organize) {
            self.current_tab = AppTab::Organize;
        }
        ui.add_space(10.0);
        if tab_btn(ui, "⚙  Settings", AppTab::Settings, current == AppTab::Settings) {
            self.current_tab = AppTab::Settings;
        }

        ui.add_space(20.0);
        // "Open Folder" button with same size as navigation buttons
        let open_btn = egui::Button::new("📂 Open Folder")
            .min_size(egui::vec2(ui.available_width(), 40.0));
        if ui.add(open_btn).clicked() {
            if let Some(path) = &self.target_dir {
                let _ = std::process::Command::new("explorer").arg(path).spawn();
            }
        }
        
        ui.add_space(10.0);
        let support_btn = egui::Button::new("💝 Support Development")
            .min_size(egui::vec2(ui.available_width(), 40.0));
        if ui.add(support_btn).clicked() {
            self.show_support_modal = true;
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            ui.label("v0.5.0");
        });
    }


    fn render_organize(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Organize Photos & Videos");
        ui.separator();

        if let Some(res) = self.organization_result.clone() {
            self.render_organization_report(ui, &res);
            return;
        }
        
        // Clone state to avoid borrow issues
        let state = self.state.clone();
        
        // State Machine for Import Wizard
        match &state {
            ProcessState::Idle => {
                ui.group(|ui| {
                    ui.label("1. Select Source Directory:");
                    ui.horizontal(|ui| {
                        if let Some(path) = &self.source_dir {
                            ui.monospace(path.to_string_lossy());
                        } else {
                            ui.label("None selected");
                        }
                        if ui.button("Choose...").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                self.source_dir = Some(path);
                            }
                        }
                    });
                });
                
                ui.add_space(20.0);
                
                let can_analyze = self.source_dir.is_some() && self.target_dir.is_some();
                if ui.add_enabled(can_analyze, egui::Button::new("Analyze Files")).clicked() {
                    self.start_analysis();
                }
                
                if self.target_dir.is_none() {
                    ui.colored_label(egui::Color32::RED, "Please configure Target Directory in Settings first.");
                }
            },
            ProcessState::Analyzing => {
                 ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.spinner();
                    ui.heading("Analyzing Source...");
                    ui.label("This may take a moment for large collections.");
                });
            },
            ProcessState::AnalyzeComplete(files) => {
                 ui.heading(format!("Analysis Complete: {} files found", files.len()));
                 ui.add_space(10.0);
                 
                 ui.horizontal(|ui| {
                     if ui.button("Cancel / Reset").clicked() {
                         self.state = ProcessState::Idle;
                     }
                     if ui.button("Import & Organize").clicked() {
                         self.show_warning_modal = true;
                     }
                 });
                 
                 ui.separator();
                 ui.label("Preview:");
                 egui::ScrollArea::vertical().show(ui, |ui| {
                    for file in files.iter().take(50) {
                         ui.label(format!("{:?}", file.path.file_name().unwrap()));
                    }
                 });
            },
            ProcessState::Organizing => {
                 ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.spinner();
                    ui.heading("Organizing Photos...");
                    ui.label("Moving files to Target directory...");
                });
            }
        }
    }

    fn render_organization_report(&mut self, ui: &mut egui::Ui, res: &OrganizationResult) {
        ui.heading("✅ Organization Complete");
        ui.add_space(10.0);
        
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.strong("Summary:");
                ui.label(format!("• Total Processed: {}", res.processed_files));
                ui.label(format!("• Files Moved: {}", res.photos_moved + res.videos_moved));
                ui.label(format!("• Duplicates Found: {}", res.duplicates_found));
                
                ui.add_space(10.0);
                ui.separator();
                ui.strong("Photos Breakdown:");
                ui.label(format!("Total Photos: {}", res.photos_moved));
                for (year, count) in &res.photos_per_year {
                    ui.label(format!("  • {}: {}", year, count));
                }

                ui.add_space(10.0);
                ui.separator();
                ui.strong("Videos Breakdown:");
                ui.label(format!("Total Videos: {}", res.videos_moved));
                for (year, count) in &res.videos_per_year {
                    ui.label(format!("  • {}: {}", year, count));
                }
            });
        });

        ui.add_space(20.0);
        if ui.button("Back to Organizer").clicked() {
            self.organization_result = None;
            self.state = ProcessState::Idle;
        }
    }

    fn render_support_modal(&mut self, ctx: &egui::Context) {
        egui::Window::new("Support this Project")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.set_width(400.0);
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    ui.label("Help me remove the \"Unknown Publisher\" warning by funding a Code Signing Certificate. Your donation makes this tool safer and more trusted for everyone.");
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        let paypal_btn = egui::Button::new("Donate with PayPal")
                            .min_size(egui::vec2(180.0, 32.0));
                        if ui.add(paypal_btn).clicked() {
                            let url = "https://www.paypal.com/donate/?business=Q4JJUB58QT7SN&no_recurring=1&item_name=Donations+help+me+purchase+a+Code+Signing+Certificate+to+remove+the+Unknown+Publisher+warning+and+build+trust+for+all+users.&currency_code=EUR";
                            let _ = std::process::Command::new("cmd").args(&["/C", "start", "", url]).spawn();
                        }
                        
                        ui.add_space(10.0);
                        
                        let ideal_btn = egui::Button::new("Donate with iDEAL")
                            .min_size(egui::vec2(180.0, 32.0));
                        if ui.add(ideal_btn).clicked() {
                            let url = "https://betaalverzoek.rabobank.nl/betaalverzoek/?id=MiDjVyNBSN-Qy288Zb0sJg";
                            let _ = std::process::Command::new("cmd").args(&["/C", "start", "", url]).spawn();
                        }
                    });
                    
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            self.show_support_modal = false;
                        }
                    });
                });
            });
    }

    fn render_warning_modal(&mut self, ctx: &egui::Context) {
        egui::Window::new("Proceed with Organization?")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.set_width(450.0);
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("⚠️ Important Safety Checklist:");
                            ui.add_space(5.0);
                            ui.label("• Backup: Please ensure you have a backup before starting.");
                            ui.label("• Duplicates: These will be safely moved to a separate /Duplicates folder.");
                            ui.label("• Unsupported Files: Files like iPhone edit info (.AAE) will remain in the source folder.");
                        });
                    });
                    
                    ui.add_space(15.0);
                    ui.colored_label(egui::Color32::from_rgb(200, 50, 50), "Use this software at your own risk.");
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Cancel").clicked() {
                                self.show_warning_modal = false;
                            }
                            
                            ui.add_space(10.0);
                            
                            let proceed_btn = egui::Button::new("Proceed");
                            if ui.add(proceed_btn).clicked() {
                                self.show_warning_modal = false;
                                self.start_organizing();
                            }
                        });
                    });
                });
            });
    }

    fn render_settings(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();
        
        ui.group(|ui| {
            ui.label("Target Directory (Library):");
            ui.horizontal(|ui| {
                if let Some(path) = &self.target_dir {
                    ui.monospace(path.to_string_lossy());
                } else {
                    ui.label("Not Configured");
                }
                if ui.button("Change...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.target_dir = Some(path);
                        self.save_config();
                    }
                }
            });
            ui.label("This is where your photos will be organized by Year.");
        });
    }

    fn render_welcome(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
             ui.vertical_centered(|ui| {
                 ui.add_space(100.0);
                 ui.heading("Welcome to Photo & Video Organizer");
                 ui.label("Let's get started. Where would you like to keep your organized photos?");
                 
                 ui.add_space(20.0);
                 
                 if ui.button("Select Photo Library Folder").clicked() {
                     if let Some(path) = rfd::FileDialog::new().pick_folder() {
                         self.target_dir = Some(path);
                         self.save_config();
                         self.is_welcome_screen = false;
                         self.current_tab = AppTab::Organize;
                     }
                 }
             });
        });
    }
}

impl App for PhotoOrganizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Poll for updates in thread channels
        self.poll_updates();
        
        if self.is_welcome_screen {
            self.render_welcome(ctx);
            return;
        }

        // Main Layout: Sidebar + Content
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(120.0)
            .show(ctx, |ui| {
                self.render_sidebar(ctx, ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                AppTab::Organize => self.render_organize(ctx, ui),
                AppTab::Settings => self.render_settings(ctx, ui),
            }
        });
        
        // Show error toast/overlay if needed
        if let Some(err) = self.error_message.clone() {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(&err);
                    if ui.button("Dismiss").clicked() {
                        self.error_message = None;
                    }
                });
        }

        if self.show_support_modal {
            self.render_support_modal(ctx);
        }

        if self.show_warning_modal {
            self.render_warning_modal(ctx);
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.save_config();
    }
}
