use raylib::prelude::*;

const WINDOW_WIDTH:  i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;
const WINDOW_TITLE: &str = "OpenVegasPro";

// ── Toolbar layout ────────────────────────────────────────────────────────────
const TOOLBAR_H:    i32 = 48;
const BTN_SIZE:     i32 = 36;
const BTN_Y:        i32 = (TOOLBAR_H - BTN_SIZE) / 2; // 6 px – vertically centred
const BTN_START_X:  i32 = 8;
const BTN_GAP:      i32 = 4;

// ── Colour palette ────────────────────────────────────────────────────────────
const C_BG:          Color = Color { r: 28,  g: 28,  b: 32,  a: 255 };
const C_TOOLBAR:     Color = Color { r: 38,  g: 38,  b: 44,  a: 255 };
const C_TOOLBAR_SEP: Color = Color { r: 60,  g: 60,  b: 68,  a: 255 };
const C_BTN:         Color = Color { r: 52,  g: 52,  b: 60,  a: 255 };
const C_BTN_HOVER:   Color = Color { r: 72,  g: 72,  b: 84,  a: 255 };
const C_BTN_PRESS:   Color = Color { r: 45,  g: 105, b: 175, a: 255 };
const C_ICON:        Color = Color { r: 205, g: 205, b: 212, a: 255 };
const C_TIP_BG:      Color = Color { r: 18,  g: 18,  b: 22,  a: 245 };
const C_TIP_BD:      Color = Color { r: 75,  g: 75,  b: 88,  a: 255 };
const C_TIP_TEXT:    Color = Color { r: 200, g: 200, b: 210, a: 255 };

// ── Button kind ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
enum BtnKind { New, Open, Save }

// ── ToolButton ────────────────────────────────────────────────────────────────
struct ToolButton {
    rect:  Rectangle,
    kind:  BtnKind,
    label: &'static str,
}

impl ToolButton {
    fn new(index: i32, kind: BtnKind, label: &'static str) -> Self {
        let x = BTN_START_X + index * (BTN_SIZE + BTN_GAP);
        Self {
            rect: Rectangle { x: x as f32, y: BTN_Y as f32,
                              width: BTN_SIZE as f32, height: BTN_SIZE as f32 },
            kind,
            label,
        }
    }

    fn hit(&self, m: Vector2) -> bool {
        m.x >= self.rect.x && m.x < self.rect.x + self.rect.width
            && m.y >= self.rect.y && m.y < self.rect.y + self.rect.height
    }

    fn draw(&self, d: &mut RaylibDrawHandle, mouse: Vector2, lmb_down: bool) {
        let hovered = self.hit(mouse);
        let pressed  = hovered && lmb_down;

        let bg = if pressed      { C_BTN_PRESS }
                 else if hovered { C_BTN_HOVER }
                 else            { C_BTN       };

        // Rounded background
        d.draw_rectangle_rounded(self.rect, 0.22, 6, bg);

        let cx = self.rect.x as i32 + BTN_SIZE / 2;
        let cy = self.rect.y as i32 + BTN_SIZE / 2;

        match self.kind {
            BtnKind::New  => icon_new(d, cx, cy),
            BtnKind::Open => icon_open(d, cx, cy),
            BtnKind::Save => icon_save(d, cx, cy),
        }

        // Tooltip – rendered below the toolbar so it never overlaps buttons
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

// ── Icons (all procedural) ────────────────────────────────────────────────────

/// New — blank page with folded top-right corner
fn icon_new(d: &mut RaylibDrawHandle, cx: i32, cy: i32) {
    let x    = cx - 7;
    let y    = cy - 9;
    let w    = 14i32;
    let h    = 18i32;
    let fold = 5i32;

    // Page body split into two rects to leave the corner free
    d.draw_rectangle(x,        y,        w - fold, fold,     C_ICON);
    d.draw_rectangle(x,        y + fold, w,        h - fold, C_ICON);

    // Folded flap (slightly darker triangle)
    d.draw_triangle(
        Vector2::new((x + w - fold) as f32, y as f32),
        Vector2::new((x + w - fold) as f32, (y + fold) as f32),
        Vector2::new((x + w) as f32,        (y + fold) as f32),
        Color { r: 155, g: 155, b: 163, a: 255 },
    );

    // Fold crease
    d.draw_line(x + w - fold, y, x + w, y + fold,
        Color { r: 135, g: 135, b: 145, a: 255 });

    // Decorative "text" lines
    let lc = Color { r: 85, g: 140, b: 210, a: 215 };
    d.draw_rectangle(x + 2, y + fold + 3,  w - 4, 2, lc);
    d.draw_rectangle(x + 2, y + fold + 7,  w - 4, 2, lc);
    d.draw_rectangle(x + 2, y + fold + 11, w - 6, 2, lc);
}

/// Open — yellow folder
fn icon_open(d: &mut RaylibDrawHandle, cx: i32, cy: i32) {
    let x = cx - 9;
    let y = cy - 6;
    let w = 18i32;
    let h = 13i32;

    let fc   = Color { r: 220, g: 175, b: 65,  a: 255 };
    let hi   = Color { r: 242, g: 202, b: 100, a: 255 };
    let sh   = Color { r: 183, g: 138, b: 42,  a: 255 };

    // Main body + tab
    d.draw_rectangle(x,  y + 4, w, h - 4, fc);
    d.draw_rectangle(x,  y,     9, 5,     fc);
    // Top highlight strip
    d.draw_rectangle(x,  y + 4, w, 3, hi);
    // Bottom shadow
    d.draw_rectangle(x,  y + h - 2, w, 2, sh);
    // Inner gleam (thin line)
    d.draw_rectangle(x + 2, y + 5, w - 4, 1, hi);
}

/// Save — classic floppy disk
fn icon_save(d: &mut RaylibDrawHandle, cx: i32, cy: i32) {
    let x      = cx - 9;
    let y      = cy - 9;
    let s      = 18i32;
    let slot_h = 7i32;

    // Outer casing
    d.draw_rectangle(x, y, s, s, C_ICON);

    // Disk slot (dark cutout at top)
    d.draw_rectangle(x + 2,     y,     s - 7,         slot_h,
        Color { r: 48, g: 48, b: 56, a: 255 });

    // Metal shutter (right side of slot)
    d.draw_rectangle(x + s - 5, y,     3,             slot_h,
        Color { r: 175, g: 175, b: 182, a: 255 });

    // Label area (white block, bottom)
    d.draw_rectangle(x + 2,     y + slot_h + 2, s - 4, s - slot_h - 4,
        Color { r: 238, g: 238, b: 242, a: 255 });

    // Label decoration lines
    let lc = Color { r: 120, g: 120, b: 180, a: 210 };
    d.draw_rectangle(x + 4, y + slot_h + 4, s - 8,  2, lc);
    d.draw_rectangle(x + 4, y + slot_h + 8, s - 10, 2, lc);
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(WINDOW_TITLE)
        .vsync()
        .resizable()
        .build();

    rl.set_target_fps(60);

    let buttons = [
        ToolButton::new(0, BtnKind::New,  "New  (Ctrl+N)"),
        ToolButton::new(1, BtnKind::Open, "Open (Ctrl+O)"),
        ToolButton::new(2, BtnKind::Save, "Save (Ctrl+S)"),
    ];

    while !rl.window_should_close() {
        // ── Input / update ────────────────────────────────────────────────────
        let mouse     = rl.get_mouse_position();
        let lmb_down  = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let lmb_click = rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);

        let ctrl = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
                || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);

        if ctrl {
            if rl.is_key_pressed(KeyboardKey::KEY_N) { println!("[New]");  }
            if rl.is_key_pressed(KeyboardKey::KEY_O) { println!("[Open]"); }
            if rl.is_key_pressed(KeyboardKey::KEY_S) { println!("[Save]"); }
        }

        if lmb_click {
            for btn in &buttons {
                if btn.hit(mouse) {
                    let name = match btn.kind {
                        BtnKind::New  => "New",
                        BtnKind::Open => "Open",
                        BtnKind::Save => "Save",
                    };
                    println!("[{}]", name);
                }
            }
        }

        let sw = rl.get_screen_width();
        let sh = rl.get_screen_height();

        // ── Draw ──────────────────────────────────────────────────────────────
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(C_BG);

        // ── Toolbar ───────────────────────────────────────────────────────────
        d.draw_rectangle(0, 0, sw, TOOLBAR_H, C_TOOLBAR);
        d.draw_rectangle(0, TOOLBAR_H - 1, sw, 1, C_TOOLBAR_SEP);

        for btn in &buttons {
            btn.draw(&mut d, mouse, lmb_down);
        }

        // App name – right-aligned inside toolbar
        let app_label = "OpenVegasPro";
        let label_w = d.measure_text(app_label, 16);
        d.draw_text(app_label, sw - label_w - 12, (TOOLBAR_H - 16) / 2,
            16, Color { r: 110, g: 110, b: 122, a: 220 });

        // FPS – bottom-right
        d.draw_fps(sw - 80, sh - 26);

        // ── Centre greeting ───────────────────────────────────────────────────
        let msg   = "Welcome to OpenVegasPro";
        let msg_w = d.measure_text(msg, 40);
        d.draw_text(msg, (sw - msg_w) / 2, (sh + TOOLBAR_H) / 2 - 34,
            40, Color::WHITE);

        let sub   = "A video editor powered by Rust + raylib + OpenGL";
        let sub_w = d.measure_text(sub, 20);
        d.draw_text(sub, (sw - sub_w) / 2, (sh + TOOLBAR_H) / 2 + 16,
            20, Color { r: 165, g: 165, b: 175, a: 255 });
    }
}

