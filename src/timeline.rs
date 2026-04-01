use raylib::prelude::*;
use crate::keybindings::Action;
use std::path::Path;

#[derive(Clone, PartialEq, Eq)]
pub enum TrackType {
    Video, // Video or Image
    Audio,
}

pub struct TimelineItem {
    pub source_path: String,
    pub start_time: f32, // in seconds
    pub duration: f32,
    pub media_offset: f32, // Where in the source media to start playing from
}

impl TimelineItem {
    // Check if this item is active at a given timeline position
    pub fn is_active_at(&self, time: f32) -> bool {
        time >= self.start_time && time < self.start_time + self.duration
    }
    
    // Get the local media time corresponding to a timeline position
    pub fn get_local_time(&self, timeline_position: f32) -> f32 {
        (timeline_position - self.start_time) + self.media_offset
    }
}

pub struct Track {
    pub name: String,
    pub kind: TrackType,
    pub items: Vec<TimelineItem>,
}

pub struct Timeline {
    pub position: f32,
    pub duration: f32,
    pub zoom: f32,
    pub is_playing: bool,
    pub play_start_pos: f32,
    pub tracks: Vec<Track>,
    pub dragging_item: Option<(usize, usize, f32)>, // track_idx, item_idx, time_offset_from_mouse
}

impl Timeline {
    pub fn new() -> Self {
        Self { 
            position: 0.0, 
            duration: 120.0, 
            zoom: 1.0, 
            is_playing: false, 
            play_start_pos: 0.0,
            tracks: Vec::new(),
            dragging_item: None,
        }
    }

    pub fn handle_drop(&mut self, path: String, mouse_x: f32, mouse_y: f32, view_x: i32, view_y: i32, view_w: i32, view_h: i32) {
        // Only if dropped in timeline rect
        if mouse_x < view_x as f32 || mouse_x > (view_x + view_w) as f32 
            || mouse_y < view_y as f32 || mouse_y > (view_y + view_h) as f32 {
            return;
        }

        let p = Path::new(&path);
        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        
        let (kind, duration) = match ext.as_str() {
            "mp4" | "mov" | "avi" | "mkv" | "webm" => (TrackType::Video, 5.0),
            "png" | "jpg" | "jpeg" | "bmp" => (TrackType::Video, 5.0),
            "mp3" | "wav" | "ogg" | "flac" => (TrackType::Audio, 5.0), // Need actual duration parsing later
            _ => { return; } // Unsupported
        };

        // Determine drop time based on mouse X tracking relative to body_x
        let header_w = 120;
        let body_x = view_x + header_w;
        let body_w = view_w - header_w;
        
        let drop_time = ((mouse_x - body_x as f32) / (body_w as f32).max(1.0) * self.duration).clamp(0.0, self.duration);

        // Find or create correct track
        let tracks_start_y = view_y + 10 + 24; // ruler is at view_y + 10 with height 24
        let track_list_h = view_h - 34;
        let track_h = 40;
        let relative_y = mouse_y - tracks_start_y as f32;
        let mut track_idx = None;
        
        if relative_y > 0.0 && relative_y < track_list_h as f32 {
            let idx = (relative_y / track_h as f32) as usize;
            if idx < self.tracks.len() && self.tracks[idx].kind == kind {
                track_idx = Some(idx);
            }
        }

        if track_idx.is_none() {
            // Check if we have an empty or last track of this type we can add to
            for (i, t) in self.tracks.iter().enumerate() {
                if t.kind == kind {
                    track_idx = Some(i);
                    // Could also continue to find the bottom-most track
                }
            }
        }

        if track_idx.is_none() {
            // Create new track
            let t_name = if kind == TrackType::Video { "Video Track" } else { "Audio Track" };
            self.tracks.push(Track {
                name: format!("{} {}", t_name, self.tracks.len() + 1),
                kind,
                items: Vec::new(),
            });
            track_idx = Some(self.tracks.len() - 1);
        }

        if let Some(idx) = track_idx {
            self.tracks[idx].items.push(TimelineItem {
                source_path: path.clone(),
                start_time: drop_time,
                duration,
                media_offset: 0.0,
            });
        }
    }

    // Get the top-most video frame available at the given time
    pub fn get_active_video_item_at(&self, time: f32) -> Option<&TimelineItem> {
        for track in self.tracks.iter().rev() { // Top-down logic
            if track.kind != TrackType::Video { continue; }
            for item in &track.items {
                if item.is_active_at(time) {
                    return Some(item);
                }
            }
        }
        None
    }

    // Get all audio clips active at the given time (we mix them)
    pub fn get_active_audio_items_at(&self, time: f32) -> Vec<&TimelineItem> {
        let mut active_audio = Vec::new();
        for track in &self.tracks {
            if track.kind != TrackType::Audio { continue; }
            for item in &track.items {
                if item.is_active_at(time) {
                    active_audio.push(item);
                }
            }
        }
        active_audio
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_playing {
            self.position += dt;
            if self.position >= self.duration {
                self.position = self.duration;
                self.is_playing = false; // Stop at end
            }
        }
    }

    pub fn toggle_play_pause(&mut self) {
        if self.is_playing {
            // Spacebar behavior: just pause at current position
            self.is_playing = false;
        } else {
            // Start playing
            self.play_start_pos = self.position;
            self.is_playing = true;
        }
    }

    pub fn stop(&mut self) {
        // Enter behavior: stop and return to where it started playing
        self.is_playing = false;
        self.position = self.play_start_pos;
    }

    pub fn go_to_start(&mut self) {
        self.position = 0.0;
        self.is_playing = false;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, x: i32, y: i32, w: i32, h: i32, mouse: Vector2, lmb_down: bool, lmb_click: bool) -> Option<Action> {
        d.draw_rectangle(x, y, w, h, Color::new(30, 32, 40, 255));
        d.draw_rectangle_lines(x, y, w, h, Color::new(75, 75, 85, 255));

        let header_w = 120;
        let body_x = x + header_w;
        let body_w = w - header_w;

        // Ruler
        let ruler_y = y + 10;
        let ruler_h = 24;
        d.draw_rectangle(x, ruler_y, header_w, ruler_h, Color::new(35, 37, 45, 255));
        d.draw_rectangle(body_x, ruler_y, body_w, ruler_h, Color::new(42, 44, 52, 255));
        d.draw_rectangle_lines(body_x, ruler_y, body_w, ruler_h, Color::new(105, 105, 130, 255));

        let num_markers = 15;
        for i in 0..=num_markers {
            let fx = body_x + (body_w * i) / num_markers;
            d.draw_line(fx, ruler_y, fx, ruler_y + ruler_h, Color::new(80, 80, 90, 255));
            let seconds = (self.duration / num_markers as f32) * i as f32;
            let label = format!("{:2.1}s", seconds);
            d.draw_text(&label, fx + 4, ruler_y + 4, 10, Color::new(180, 180, 200, 255));
        }

        // Draw Tracks
        let tracks_start_y = ruler_y + ruler_h;
        let track_row_h = 40;
        let pixels_per_sec = body_w as f32 / self.duration;

        let mut drag_hover_found = false;

        for (i, track) in self.tracks.iter_mut().enumerate() {
            let ty = tracks_start_y + i as i32 * track_row_h;
            if ty + track_row_h > y + h - 30 { break; } // don't draw over controls

            // Track Header
            d.draw_rectangle(x, ty, header_w, track_row_h, Color::new(45, 48, 55, 255));
            d.draw_rectangle_lines(x, ty, header_w, track_row_h, Color::new(105, 105, 130, 255));
            d.draw_text(&track.name, x + 8, ty + 14, 12, Color::new(220, 220, 220, 255));
            
            // Track Body
            d.draw_rectangle(body_x, ty, body_w, track_row_h, Color::new(35, 38, 45, 255));
            d.draw_rectangle_lines(body_x, ty, body_w, track_row_h, Color::new(80, 80, 95, 255));

            // Track Items
            for (item_idx, item) in track.items.iter_mut().enumerate() {
                let is_being_dragged = self.dragging_item == Some((i, item_idx, 0.0)) || self.dragging_item.is_some_and(|dr| dr.0 == i && dr.1 == item_idx);
                
                // If dragging, update position
                if is_being_dragged && lmb_down {
                    let mouse_time = ((mouse.x - body_x as f32) / pixels_per_sec).max(0.0);
                    let (_t, _i, offset) = self.dragging_item.unwrap();
                    item.start_time = (mouse_time + offset).max(0.0);
                }

                let item_x = body_x + (item.start_time * pixels_per_sec) as i32;
                let item_w = (item.duration * pixels_per_sec) as i32;
                
                let (bg_color, outline) = match track.kind {
                    TrackType::Video => (Color::new(70, 110, 160, 255), Color::new(100, 140, 190, 255)),
                    TrackType::Audio => (Color::new(70, 140, 100, 255), Color::new(100, 170, 130, 255)),
                };

                let item_rect = Rectangle::new(item_x as f32, (ty + 2) as f32, item_w as f32, (track_row_h - 4) as f32);
                let is_hovered = mouse.x >= item_rect.x && mouse.x <= item_rect.x + item_rect.width 
                              && mouse.y >= item_rect.y && mouse.y <= item_rect.y + item_rect.height;

                // Capture drag
                if is_hovered && lmb_down && self.dragging_item.is_none() && !drag_hover_found {
                    if d.get_mouse_delta().length() > 0.0 {
                        let mouse_time = ((mouse.x - body_x as f32) / pixels_per_sec).max(0.0);
                        self.dragging_item = Some((i, item_idx, item.start_time - mouse_time));
                        drag_hover_found = true;
                    }
                }

                let draw_bg = if is_being_dragged { Color::new(100, 140, 200, 255) } else { bg_color };

                d.draw_rectangle_rec(item_rect, draw_bg);
                d.draw_rectangle_lines_ex(item_rect, 1.0, outline);

                // Filename
                let fname = Path::new(&item.source_path).file_name().and_then(|n| n.to_str()).unwrap_or("");
                // Only draw text if it fits loosely
                if item_w > 20 {
                    d.draw_text(fname, item_x + 4, ty + 12, 10, Color::new(240, 240, 255, 255));
                }
            }
        }

        if !lmb_down {
            self.dragging_item = None;
        }

        // Playhead
        let pos_x_f = body_x as f32 + (body_w as f32 * (self.position / self.duration)).clamp(0.0, body_w as f32);
        let pos_x = pos_x_f as i32;
        let timeline_bottom = y + h - 30; // Above controls
        d.draw_line(pos_x, ruler_y, pos_x, timeline_bottom, Color::new(220, 75, 60, 255));
        d.draw_triangle(
            Vector2::new(pos_x as f32 - 5.0, ruler_y as f32),
            Vector2::new(pos_x as f32 + 5.0, ruler_y as f32),
            Vector2::new(pos_x as f32, ruler_y as f32 + 8.0),
            Color::new(220, 75, 60, 255)
        );

        // Click on Ruler to scrub time
        let over_ruler = mouse.x >= body_x as f32 && mouse.x <= (body_x + body_w) as f32 
                      && mouse.y >= ruler_y as f32 && mouse.y <= timeline_bottom as f32;
        if over_ruler && (lmb_click || lmb_down) {
            self.position = ((mouse.x - body_x as f32) / pixels_per_sec).clamp(0.0, self.duration);
            if !self.is_playing && lmb_click {
                 self.play_start_pos = self.position;
            }
        }

        // Let's add playback control buttons: [|<<] [|>] [||] [[]]
        let mut triggered_action = None;
        let btn_w = 28;
        let btn_h = 24;
        let start_x = x + 12;
        let bottom_y = y + h - btn_h - 10;

        let controls = [
            ("|<", Action::GoToStart, 0),
            (if self.is_playing { "||" } else { ">" }, Action::PlayPause, 1),
            ("[]", Action::Stop, 2),
        ];

        for (label, action, idx) in &controls {
            let btn_x = start_x + idx * (btn_w + 4);
            let rect = Rectangle::new(btn_x as f32, bottom_y as f32, btn_w as f32, btn_h as f32);
            
            let hovered = mouse.x >= rect.x && mouse.x <= rect.x + rect.width 
                       && mouse.y >= rect.y && mouse.y <= rect.y + rect.height;

            let bg = if hovered { Color::new(70, 75, 90, 255) } else { Color::new(50, 52, 60, 255) };
            d.draw_rectangle_rec(rect, bg);
            d.draw_rectangle_lines_ex(rect, 1.0, Color::new(90, 95, 110, 255));

            let lw = d.measure_text(label, 12);
            d.draw_text(label, btn_x + (btn_w - lw)/2, bottom_y + 6, 12, Color::WHITE);

            if hovered && lmb_click {
                triggered_action = Some(*action);
            }
        }

        let text = format!("position: {:.2}s / {:.2}s", self.position, self.duration);
        d.draw_text(&text, start_x + (3 * (btn_w + 4)) + 12, bottom_y + 5, 14, Color::new(190, 190, 210, 255));

        triggered_action
    }
}
