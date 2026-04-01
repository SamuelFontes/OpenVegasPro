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
        Action::UntieClips => timeline.untie_selected(),
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
        
        let current_time = timeline.position;
        // Render Mixed Media Engine (Mock Video)
        if let Some(active_video) = timeline.get_active_video_item_at(current_time) {
            let local_time = active_video.get_local_time(current_time);
            let fname = std::path::Path::new(&active_video.source_path).file_name().and_then(|n| n.to_str()).unwrap_or("");
            
            // Draw placeholder video frame
            d.draw_rectangle(preview_x + 20, top_panel_y + 80, preview_w - 40, mid_panel_h - 100, Color::new(10, 10, 12, 255));
            let mut info_text = format!("Playing: {}\nTimeline Pos: {:.2}s\nLocal Media Time: {:.2}s", fname, current_time, local_time);
            
            let active_audio = timeline.get_active_audio_items_at(current_time);
            if !active_audio.is_empty() {
                info_text.push_str("\n\nMixed Audio Tracks:");
                for au in active_audio {
                    let aname = std::path::Path::new(&au.source_path).file_name().and_then(|n| n.to_str()).unwrap_or("");
                    info_text.push_str(&format!("\n• {} @ {:.2}s", aname, au.get_local_time(current_time)));
                }
            }

            d.draw_text(&info_text, preview_x + 30, top_panel_y + 95, 16, Color::new(200, 200, 220, 255));
        } else {
            d.draw_text("No active video frames at this position.", preview_x + 12, top_panel_y + 42, 16, Color::new(175, 175, 195, 255));
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
            let th_size = 32;
            let label = std::path::Path::new(dragged).file_name().and_then(|n| n.to_str()).unwrap_or("Dragging...");
            let rect_w = th_size + 12 + d.measure_text(label, 12) + 12;
            let mx = mouse.x as i32 + 10;
            let my = mouse.y as i32 + 10;
            let th_margin = 6;
            let rect_h = th_size + th_margin*2; // 44

            d.draw_rectangle(mx, my, rect_w, rect_h, Color::new(70, 90, 145, 230));
            d.draw_rectangle_lines(mx, my, rect_w, rect_h, Color::new(100, 120, 180, 255));
            
            media::draw_media_thumbnail(&mut d, dragged, mx + th_margin, my + th_margin, th_size);
            
            d.draw_text(label, mx + th_margin + th_size + 8, my + (rect_h - 12)/2, 12, Color::WHITE);
        }

        drop(d); // Drop the draw handle so we can mutate timeline again

        if let Some(action) = timeline_action {
            handle_action(action, &mut timeline);
        }

        // Handle add button action from media browser
        if media_browser.add_clicked {
            if let Some(file_path) = FileDialog::new().add_filter("Media", &["mp4", "mov", "avi", "mkv", "webm", "png", "jpg", "jpeg", "bmp", "mp3", "wav", "ogg", "flac"]).pick_file() {
                if let Some(path_str) = file_path.to_str() {
                    media_browser.add(path_str.to_string());
                    println!("Added media: {}", path_str);
                }
            }
        }
    }
}
