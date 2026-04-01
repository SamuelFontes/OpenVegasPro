mod theme;
mod toolbar;
mod media;
mod timeline;
mod keybindings;

use raylib::prelude::*;
use rfd::FileDialog;
use theme::*;
use toolbar::Toolbar;
use media::MediaBrowser;
use timeline::Timeline;
use keybindings::{KeyManager, Action};

fn handle_action(action: Action, timeline: &mut Timeline) {
    match action {
        Action::PlayPause => timeline.toggle_play_pause(),
        Action::Stop => timeline.stop(),
        Action::GoToStart => timeline.go_to_start(),
        Action::NewProject => println!("[New Project]"),
        Action::OpenProject => println!("[Open Project]"),
        Action::SaveProject => println!("[Save Project]"),
        Action::SplitClip => println!("[Split Clip]"),
        Action::Undo => println!("[Undo]"),
        Action::Redo => println!("[Redo]"),
    }
}

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
    let key_manager = KeyManager::new();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        timeline.update(dt);

        // ── Input ─────────────────────────────────────────────────────────────
        let mouse     = rl.get_mouse_position();
        let lmb_down  = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let lmb_click = rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);

        let mut incoming_actions = key_manager.check_actions(&rl);

        if let Some(action) = toolbar.handle_click(mouse, lmb_click) {
            let a = match action {
                toolbar::BtnKind::New => Action::NewProject,
                toolbar::BtnKind::Open => Action::OpenProject,
                toolbar::BtnKind::Save => Action::SaveProject,
            };
            incoming_actions.push(a);
        }

        // Process pre-draw actions
        for action in incoming_actions {
            handle_action(action, &mut timeline);
        }

        let sw = rl.get_screen_width();
        let sh = rl.get_screen_height();
        
        // global file drop into window
        if rl.is_file_dropped() {
            let dropped = rl.load_dropped_files();
            let timeline_y = sh - 160; // bottom_timeline_h = 160;

            for path in dropped.paths() {
                let path_str = path.to_string();
                if mouse.y >= timeline_y as f32 {
                    // Drop into timeline
                    timeline.handle_drop(path_str.clone(), mouse.x, mouse.y, 0, timeline_y, sw, 160);
                }
                
                // Add to media browser always (Vegas logic)
                media_browser.add(path_str);
                println!("Dropped: {}", path.to_string());
            }
        }

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
            let fname = std::path::Path::new(&media_browser.items[selected]).file_name().and_then(|n| n.to_str()).unwrap_or("");
            let label = format!("Playing: {}", fname);
            d.draw_text(&label, preview_x + 12, top_panel_y + 42, 16, Color::new(190, 190, 210, 255));
        } else {
            d.draw_text("No media selected.", preview_x + 12, top_panel_y + 42, 16, Color::new(175, 175, 195, 255));
        }

        // Timeline (bottom)
        let timeline_y = sh - bottom_timeline_h;
        let mut timeline_action = None;
        if let Some(action) = timeline.draw(&mut d, 0, timeline_y, sw, bottom_timeline_h, mouse, lmb_down, lmb_click) {
            timeline_action = Some(action);
        }

        // Drag media from browser to timeline
        if lmb_click && media_browser.dragging_item.is_some() {
            if mouse.y >= timeline_y as f32 {
                let dragged_path = media_browser.dragging_item.as_ref().unwrap().clone();
                timeline.handle_drop(dragged_path, mouse.x, mouse.y, 0, timeline_y, sw, bottom_timeline_h);
            }
            media_browser.dragging_item = None; // Drop handled
        }

        // Drag-and-drop hint
        d.draw_text("Drag files from your OS into the window to import into media browser.", 12, sh - bottom_timeline_h - 24, 14, Color::new(160, 160, 175, 255));

        // FPS – bottom-right
        d.draw_fps(sw - 80, sh - 26);
        
        // Draw MediaBrowser dragged item thumbnail ON TOP OF EVERYTHING
        if let Some(ref dragged) = media_browser.dragging_item {
            let label = std::path::Path::new(dragged).file_name().and_then(|n| n.to_str()).unwrap_or("Dragging...");
            let rect_w = d.measure_text(label, 12) + 24;
            d.draw_rectangle(mouse.x as i32 + 10, mouse.y as i32 + 10, rect_w, 26, Color::new(70, 90, 145, 180));
            d.draw_text(label, mouse.x as i32 + 16, mouse.y as i32 + 16, 12, Color::WHITE);
        }

        drop(d); // Drop the draw handle so we can mutate timeline again

        if let Some(action) = timeline_action {
            handle_action(action, &mut timeline);
        }

        // Handle add button action from media browser
        if media_browser.add_clicked {
            if let Some(file_path) = FileDialog::new().add_filter("Media", &["mp4", "mov", "avi", "mkv", "png", "jpg"]).pick_file() {
                if let Some(path_str) = file_path.to_str() {
                    media_browser.add(path_str.to_string());
                    println!("Added media: {}", path_str);
                }
            }
        }
    }
}
