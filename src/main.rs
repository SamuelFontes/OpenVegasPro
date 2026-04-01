mod theme;
mod toolbar;
mod media;
mod timeline;

use raylib::prelude::*;
use rfd::FileDialog;
use theme::*;
use toolbar::Toolbar;
use media::MediaBrowser;
use timeline::Timeline;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(WINDOW_TITLE)
        .vsync()
        .resizable()
        .build();

    rl.set_target_fps(60);

    let toolbar = Toolbar::new();
    let mut media_browser = MediaBrowser::new();
    let mut timeline = Timeline::new();

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

        // global file drop into window
        if rl.is_file_dropped() {
            let dropped = rl.load_dropped_files();
            for path in dropped.paths() {
                media_browser.add(path.to_string());
                println!("Dropped: {}", path);
            }
        }

        let sw = rl.get_screen_width();
        let sh = rl.get_screen_height();

        // ── Draw ──────────────────────────────────────────────────────────────
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(C_BG);

        toolbar.draw(&mut d, mouse, lmb_down, sw);

        // Layout
        let left_panel_w = 360;
        let top_panel_y = TOOLBAR_H;
        let bottom_timeline_h = 160;
        let mid_panel_h = sh - top_panel_y - bottom_timeline_h;

        // Media browser (left + top)
        media_browser.draw(&mut d, 0, top_panel_y, left_panel_w, mid_panel_h, mouse, lmb_down, lmb_click);

        // Preview area (right + top)
        let preview_x = left_panel_w;
        let preview_w = sw - left_panel_w;
        d.draw_rectangle(preview_x, top_panel_y, preview_w, mid_panel_h, Color::new(26, 28, 34, 255));
        d.draw_rectangle_lines(preview_x, top_panel_y, preview_w, mid_panel_h, Color::new(92, 92, 102, 255));

        d.draw_text("Video Preview", preview_x + 12, top_panel_y + 12, 20, Color::new(220, 220, 230, 255));
        if let Some(selected) = media_browser.selected {
            let label = format!("Playing: {}", media_browser.items[selected]);
            d.draw_text(&label, preview_x + 12, top_panel_y + 42, 16, Color::new(190, 190, 210, 255));
        } else {
            d.draw_text("No media selected.", preview_x + 12, top_panel_y + 42, 16, Color::new(175, 175, 195, 255));
        }

        // Timeline (bottom)
        timeline.draw(&mut d, 0, sh - bottom_timeline_h, sw, bottom_timeline_h);

        // Drag-and-drop hint
        d.draw_text("Drag files from your OS into the window to import into media browser.", 12, sh - bottom_timeline_h - 24, 14, Color::new(160, 160, 175, 255));

        // Handle add button action from media browser
        if media_browser.add_clicked {
            if let Some(file_path) = FileDialog::new().add_filter("Media", &["mp4", "mov", "avi", "mkv", "png", "jpg"]).pick_file() {
                if let Some(path_str) = file_path.to_str() {
                    media_browser.add(path_str.to_string());
                    println!("Added media: {}", path_str);
                }
            }
        }

        // FPS – bottom-right
        d.draw_fps(sw - 80, sh - 26);
    }
}
