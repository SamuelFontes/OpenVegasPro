use raylib::prelude::*;
use crate::theme::*;
use super::icons::{icon_new, icon_open, icon_save};

// ── Button kind ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum BtnKind { New, Open, Save }

impl BtnKind {
    pub fn name(self) -> &'static str {
        match self {
            BtnKind::New  => "New",
            BtnKind::Open => "Open",
            BtnKind::Save => "Save",
        }
    }
}

// ── ToolButton ────────────────────────────────────────────────────────────────
pub struct ToolButton {
    pub rect:  Rectangle,
    pub kind:  BtnKind,
    pub label: &'static str,
}

impl ToolButton {
    pub fn new(index: i32, kind: BtnKind, label: &'static str) -> Self {
        let x = BTN_START_X + index * (BTN_SIZE + BTN_GAP);
        Self {
            rect: Rectangle {
                x: x as f32,
                y: BTN_Y as f32,
                width:  BTN_SIZE as f32,
                height: BTN_SIZE as f32,
            },
            kind,
            label,
        }
    }

    pub fn hit(&self, m: Vector2) -> bool {
        m.x >= self.rect.x && m.x < self.rect.x + self.rect.width
            && m.y >= self.rect.y && m.y < self.rect.y + self.rect.height
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, mouse: Vector2, lmb_down: bool) {
        let hovered = self.hit(mouse);
        let pressed = hovered && lmb_down;

        let bg = if pressed      { C_BTN_PRESS }
                 else if hovered { C_BTN_HOVER }
                 else            { C_BTN       };

        d.draw_rectangle_rounded(self.rect, 0.22, 6, bg);

        let cx = self.rect.x as i32 + BTN_SIZE / 2;
        let cy = self.rect.y as i32 + BTN_SIZE / 2;

        match self.kind {
            BtnKind::New  => icon_new(d, cx, cy),
            BtnKind::Open => icon_open(d, cx, cy),
            BtnKind::Save => icon_save(d, cx, cy),
        }

        // Tooltip — rendered just below the toolbar
        if hovered && !pressed {
            let tx = self.rect.x as i32;
            let ty = TOOLBAR_H + 6;
            let tw = d.measure_text(self.label, 12) + 12;
            d.draw_rectangle(tx, ty, tw, 22, C_TIP_BG);
            d.draw_rectangle_lines(tx, ty, tw, 22, C_TIP_BD);
            d.draw_text(self.label, tx + 6, ty + 5, 12, C_TIP_TEXT);
        }
    }
}
