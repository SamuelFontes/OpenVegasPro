use raylib::prelude::Color;

// ── Window defaults ───────────────────────────────────────────────────────────
pub const WINDOW_WIDTH:  i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;
pub const WINDOW_TITLE: &str = "OpenVegasPro";

// ── Toolbar layout ─────────────────────────────────────────────────────────────
pub const TOOLBAR_H:   i32 = 48;
pub const BTN_SIZE:    i32 = 36;
pub const BTN_Y:       i32 = (TOOLBAR_H - BTN_SIZE) / 2; // 6 px – vertically centred
pub const BTN_START_X: i32 = 8;
pub const BTN_GAP:     i32 = 4;

// ── Colour palette ─────────────────────────────────────────────────────────────
pub const C_BG:          Color = Color { r: 28,  g: 28,  b: 32,  a: 255 };
pub const C_TOOLBAR:     Color = Color { r: 38,  g: 38,  b: 44,  a: 255 };
pub const C_TOOLBAR_SEP: Color = Color { r: 60,  g: 60,  b: 68,  a: 255 };
pub const C_BTN:         Color = Color { r: 52,  g: 52,  b: 60,  a: 255 };
pub const C_BTN_HOVER:   Color = Color { r: 72,  g: 72,  b: 84,  a: 255 };
pub const C_BTN_PRESS:   Color = Color { r: 45,  g: 105, b: 175, a: 255 };
pub const C_ICON:        Color = Color { r: 205, g: 205, b: 212, a: 255 };
pub const C_TIP_BG:      Color = Color { r: 18,  g: 18,  b: 22,  a: 245 };
pub const C_TIP_BD:      Color = Color { r: 75,  g: 75,  b: 88,  a: 255 };
pub const C_TIP_TEXT:    Color = Color { r: 200, g: 200, b: 210, a: 255 };
