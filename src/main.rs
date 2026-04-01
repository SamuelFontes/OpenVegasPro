mod theme;
mod toolbar;

use raylib::prelude::*;
use theme::*;
use toolbar::Toolbar;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(WINDOW_TITLE)
        .vsync()
        .resizable()
        .build();

    rl.set_target_fps(60);

    let toolbar = Toolbar::new();

    while !rl.window_should_close() {
        // ── Input ─────────────────────────────────────────────────────────────
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

        if let Some(action) = toolbar.handle_click(mouse, lmb_click) {
            println!("[{}]", action.name());
        }

        let sw = rl.get_screen_width();
        let sh = rl.get_screen_height();

        // ── Draw ──────────────────────────────────────────────────────────────
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(C_BG);

        toolbar.draw(&mut d, mouse, lmb_down, sw);

        // Centre greeting
        let msg   = "Welcome to OpenVegasPro";
        let msg_w = d.measure_text(msg, 40);
        d.draw_text(msg, (sw - msg_w) / 2, (sh + TOOLBAR_H) / 2 - 34,
            40, Color::WHITE);

        let sub   = "A video editor powered by Rust + raylib + OpenGL";
        let sub_w = d.measure_text(sub, 20);
        d.draw_text(sub, (sw - sub_w) / 2, (sh + TOOLBAR_H) / 2 + 16,
            20, Color { r: 165, g: 165, b: 175, a: 255 });

        // FPS – bottom-right
        d.draw_fps(sw - 80, sh - 26);
    }
}
