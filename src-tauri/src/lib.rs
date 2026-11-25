use eframe::egui;

const BG: egui::Color32 = egui::Color32::from_rgb(25, 26, 31);
const PANEL: egui::Color32 = egui::Color32::from_rgb(31, 33, 40);
const PANEL_DARK: egui::Color32 = egui::Color32::from_rgb(20, 21, 26);
const ACCENT: egui::Color32 = egui::Color32::from_rgb(88, 101, 242);
const TEXT: egui::Color32 = egui::Color32::from_rgb(235, 236, 239);
const MUTED: egui::Color32 = egui::Color32::from_rgb(148,  is 155, 166);

struct BlemmcordApp {
    selected_channel: String,
    message: String,
}

impl Default for BlemmcordApp {
    fn default() -> Self {
        Self {
            selected_channel: "general".to_owned(),
            message: String::new(),
        }
    }
}

impl eframe::App for BlemmcordApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        let mut visuals = ctx.style().visuals.clone();
        visuals.panel_fill = BG;
        visuals.window_fill = PANEL;
        visuals.widgets.noninteractive.bg_fill = PANEL;
        visuals.selection.bg_fill = ACCENT;
        ctx.set_visuals(visuals);

        egui::SidePanel::left("servers")
            .exact_width(68.0)
            .frame(egui::Frame::new().fill(PANEL_DARK).inner_margin(egui::Margin::same(10)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(6.0);
                    server_icon(ui, "B", true);
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(10.0);
                    server_icon(ui, "+", false);
                });
            });

        egui::SidePanel::left("channels")
            .exact_width(220.0)
            .frame(egui::Frame::new().fill(PANEL).inner_margin(egui::Margin::same(14)))
            .show(ctx, |ui| {
                ui.heading("Blemmcord");
                ui.add_space(4.0);
                ui.label(egui::RichText::new("YOUR WORKSPACE").size(10.0).color(MUTED));
                ui.add_space(18.0);
                ui.label(egui::RichText::new("TEXT CHANNELS").strong().size(11.0).color(MUTED));
                ui.add_space(8.0);
                for channel in ["general", "announcements", "memes"] {
                    let selected = self.selected_channel == channel;
                    let response = ui.selectable_label(
                        selected,
                        egui::RichText::new(format!("#  {channel}")).color(if selected { TEXT } else { MUTED }),
                    );
                    if response.clicked() {
                        self.selected_channel = channel.to_owned();
                    }
                }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.separator();
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("●").color(egui::Color32::from_rgb(59, 165, 92)));
                        ui.label(egui::RichText::new("Local preview").color(MUTED).size(12.0));
                    });
                });
            });

        egui::TopBottomPanel::top("topbar")
            .frame(egui::Frame::new().fill(PANEL).inner_margin(egui::Margin::symmetric(18, 12)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(format!("#  {}", self.selected_channel));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("NATIVE PREVIEW  •  DISCORD NOT CONNECTED").size(10.0).color(MUTED));
                    });
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin::same(22)))
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        let edit = ui.add_sized(
                            [ui.available_width() - 76.0, 42.0],
                            egui::TextEdit::singleline(&mut self.message).hint_text(format!("Message #{}", self.selected_channel)),
                        );
                        let send = ui.add_sized([64.0, 42.0], egui::Button::new("Send").fill(ACCENT));
                        if send.clicked() || (edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                            self.message.clear();
                        }
                    });
                    ui.add_space(18.0);
                    ui.separator();
                    ui.add_space(16.0);
                    ui.label(egui::RichText::new("This is the beginning of the channel").strong().size(18.0).color(TEXT));
                    ui.label(egui::RichText::new("Your native Blemmcord Lite interface is running. Discord connectivity has not been enabled yet.").color(MUTED));
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new("Phase 1 • Native Rust UI").color(ACCENT).strong());
                });
            });
    }
}

fn server_icon(ui: &mut egui::Ui, label: &str, active: bool) {
    let fill = if active { ACCENT } else { egui::Color32::from_rgb(45, 47, 55) };
    let button = egui::Button::new(egui::RichText::new(label).size(19.0).strong())
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(18));
    ui.add_sized([44.0, 44.0], button);
}

pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Blemmcord Lite")
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([900.0, 600.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "Blemmcord Lite",
        options,
        Box::new(|_cc| Ok(Box::<BlemmcordApp>::default())),
    )
}
