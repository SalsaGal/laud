use eframe::{NativeOptions, run_native};
use eframe::egui::{CentralPanel, CtxRef};
use eframe::epi::{App, Frame};

struct Window {
}

impl Window {
	pub fn new() -> Self {
		Self {
		}
	}
}

impl App for Window {
	fn name(&self) -> &str {
		"Laud"
	}

	fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
		CentralPanel::default().show(ctx, |ui| {
		});
	}
}

fn main() {
	let window = Window::new();
	let native_options = NativeOptions {
		..Default::default()
	};
	run_native(Box::new(window), native_options);
}
