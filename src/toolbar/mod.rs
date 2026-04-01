pub mod button;
pub mod icons;

pub use button::{BtnKind, ToolButton};

use raylib::prelude::*;
use crate::theme::*;

// ── Toolbar ───────────────────────────────────────────────────────────────────
pub struct Toolbar {
    buttons: [ToolButton; 3],
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            buttons: [
                ToolButton::new(0, BtnKind::New,  "New  (Ctrl+N)"),
                ToolButton::new(1, BtnKind::Open, "Open (Ctrl+O)"),
                ToolButton::new(2, BtnKind::Save, "Save (Ctrl+S)"),
            ],
        }
    }

    /// Process mouse click over any button; returns the fired action if any.
    pub fn handle_click(&self, mouse: Vector2, lmb_click: bool) -> Option<BtnKind> {
        if !lmb_click { return None; }
        self.buttons.iter()
            .find(|b| b.hit(mouse))
            .map(|b| b.kind)
    }

    /// Draw toolbar background, separator, all buttons and the app label.
    pub fn draw(&self, d: &mut RaylibDrawHandle, mouse: Vector2, lmb_down: bool, sw: i32) {
        // Background + bottom separator
        d.draw_rectangle(0, 0, sw, TOOLBAR_H, C_TOOLBAR);
        d.draw_rectangle(0, TOOLBAR_H - 1, sw, 1, C_TOOLBAR_SEP);

        for btn in &self.buttons {
            btn.draw(d, mouse, lmb_down);
        }

        // App name – right-aligned
        let label = "OpenVegasPro";
        let lw = d.measure_text(label, 16);
        d.draw_text(label, sw - lw - 12, (TOOLBAR_H - 16) / 2,
            16, Color { r: 110, g: 110, b: 122, a: 220 });
    }
}
