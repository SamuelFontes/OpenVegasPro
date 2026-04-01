use raylib::prelude::*;
use crate::keybindings::Action;

pub struct Timeline {
    pub position: f32,
    pub duration: f32,
    pub zoom: f32,
    pub is_playing: bool,
    pub play_start_pos: f32, // Where playback started (for Vegas-style stop)
}

impl Timeline {
    pub fn new() -> Self {
        Self { position: 0.0, duration: 120.0, zoom: 1.0, is_playing: false, play_start_pos: 0.0 }
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

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, x: i32, y: i32, w: i32, h: i32, mouse: Vector2, lmb_click: bool) -> Option<Action> {
        d.draw_rectangle(x, y, w, h, Color::new(30, 32, 40, 255));
        d.draw_rectangle_lines(x, y, w, h, Color::new(75, 75, 85, 255));

        let track_y = y + 14;
        let track_h = 26;
        d.draw_rectangle(x + 10, track_y, w - 20, track_h, Color::new(42, 44, 52, 255));
        d.draw_rectangle_lines(x + 10, track_y, w - 20, track_h, Color::new(105, 105, 130, 255));

        let num_markers = 11;
        for i in 0..=num_markers {
            let fx = x + 10 + ((w - 20) * i) / num_markers;
            d.draw_line(fx, track_y, fx, track_y + track_h, Color::new(80, 80, 90, 255));
            let seconds = (self.duration / num_markers as f32) * i as f32;
            let label = format!("{:2.0}s", seconds);
            d.draw_text(&label, fx - 12, track_y + track_h + 4, 12, Color::new(180, 180, 200, 255));
        }

        let pos_x_f = (x + 10) as f32 + ((w - 20) as f32 * (self.position / self.duration)).clamp(0.0, (w - 20) as f32);
        let pos_x = pos_x_f as i32;
        d.draw_line(pos_x, track_y - 4, pos_x, track_y + track_h + 4, Color::new(220, 75, 60, 255));

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
