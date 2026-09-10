#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gpu;

use eframe::egui;
use gpu::GpuInfo;
use std::time::{Duration, Instant};

struct GpuXApp {
    gpu: GpuInfo,
    last_refresh: Instant,
    auto_refresh: bool,
}

impl GpuXApp {
    fn new() -> Self {
        Self {
            gpu: GpuInfo::detect(),
            last_refresh: Instant::now(),
            auto_refresh: true,
        }
    }

    fn refresh(&mut self) {
        self.gpu = GpuInfo::detect();
        self.last_refresh = Instant::now();
    }
}

impl eframe::App for GpuXApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.auto_refresh && self.last_refresh.elapsed() >= Duration::from_secs(1) {
            self.refresh();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GPU-X");
            ui.label("AMD GPU Information Utility");
            ui.separator();

            if !self.gpu.supported {
                ui.colored_label(
                    egui::Color32::from_rgb(220, 80, 80),
                    "No supported AMD RDNA 4 GPU detected.",
                );
                ui.separator();
            }

            section_header(ui, "GPU");

            info_row(ui, "Name", &self.gpu.name);
            info_row(ui, "Architecture", &self.gpu.architecture);
            info_row(ui, "Vendor", &self.gpu.vendor);
            info_row(ui, "PCI Vendor ID", &self.gpu.vendor_id);
            info_row(ui, "PCI Device ID", &self.gpu.device_id);
            info_row(ui, "PCI Location", &self.gpu.pci_location);
            info_row(ui, "GPU Type", &self.gpu.gpu_type);

            ui.add_space(10.0);

            section_header(ui, "Memory");

            info_row(ui, "VRAM", &self.gpu.vram_total);
            info_row(ui, "VRAM Used", &self.gpu.vram_used);
            info_row(ui, "VRAM Available", &self.gpu.vram_available);
            info_row(ui, "Memory Type", &self.gpu.memory_type);
            info_row(ui, "Memory Bus", &self.gpu.memory_bus);
            info_row(ui, "Memory Bandwidth", &self.gpu.memory_bandwidth);

            ui.add_space(10.0);

            section_header(ui, "Clocks");

            info_row(ui, "GPU Clock", &self.gpu.gpu_clock);
            info_row(ui, "Memory Clock", &self.gpu.memory_clock);
            info_row(ui, "Boost Clock", &self.gpu.boost_clock);

            ui.add_space(10.0);

            section_header(ui, "Sensors");

            info_row(ui, "GPU Usage", &self.gpu.gpu_usage);
            info_row(ui, "Temperature", &self.gpu.temperature);
            info_row(ui, "Power", &self.gpu.power);
            info_row(ui, "Fan", &self.gpu.fan);

            ui.add_space(10.0);

            section_header(ui, "Software");

            info_row(ui, "Driver", &self.gpu.driver);
            info_row(ui, "Driver Version", &self.gpu.driver_version);
            info_row(ui, "VBIOS", &self.gpu.vbios);

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Refresh").clicked() {
                    self.refresh();
                }

                ui.checkbox(&mut self.auto_refresh, "Auto refresh");

                if ui.button("Copy Information").clicked() {
                    ctx.copy_text(self.gpu.as_text());
                }
            });

            ui.add_space(5.0);

            ui.small(format!(
                "Last refresh: {:.1}s ago",
                self.last_refresh.elapsed().as_secs_f32()
            ));

            ui.small("GPU-X 0.1.0 • Read-only");
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn section_header(ui: &mut egui::Ui, text: &str) {
    ui.heading(text);
    ui.separator();
}

fn info_row(ui: &mut egui::Ui, name: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.add_sized([150.0, 20.0], egui::Label::new(
            egui::RichText::new(name).strong()
        ));

        ui.label(value);
    });
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("GPU-X")
            .with_inner_size([620.0, 760.0])
            .with_min_inner_size([500.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "GPU-X",
        options,
        Box::new(|_cc| Ok(Box::new(GpuXApp::new()))),
    )
}