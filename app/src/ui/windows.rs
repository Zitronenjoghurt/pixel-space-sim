use crate::ui::widgets::toggle_button::ToggleButton;
use egui::{Context, Id, Ui, Widget, WidgetText};

mod debug;
mod draw;
pub mod main;

pub trait UiWindow: Sized {
    fn id() -> Id;
    fn title() -> impl Into<WidgetText>;
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);
    fn render_content(&mut self, ui: &mut Ui);

    fn resizable(&self) -> bool {
        true
    }

    fn movable(&self) -> bool {
        true
    }

    fn collapsible(&self) -> bool {
        false
    }

    fn show(mut self, ctx: &Context) {
        let mut is_open = self.is_open();
        egui::Window::new(Self::title())
            .id(Self::id())
            .open(&mut is_open)
            .resizable(self.resizable())
            .movable(self.movable())
            .collapsible(self.collapsible())
            .show(ctx, |ui| self.render_content(ui));
        self.set_open(is_open && self.is_open());
    }
}

pub trait ToggleableUiWindow: UiWindow {
    fn toggle_label(&self) -> String;

    fn toggle_button(mut self, ui: &mut Ui) -> Self {
        let mut is_open = self.is_open();
        ToggleButton::new(&mut is_open, &self.toggle_label()).ui(ui);
        self.set_open(is_open);
        self
    }
}
