use raylib::prelude::*;

pub struct Timeline {
    pub position: f32,
    pub duration: f32,
    pub zoom: f32,
}

impl Timeline {
    pub fn new() -> Self {
        Self { position: 0.0, duration: 120.0, zoom: 1.0 }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, w: i32, h: i32) {
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

        let text = format!("position: {:.1}s / {:.1}s", self.position, self.duration);
        d.draw_text(&text, x + 12, y + h - 26, 14, Color::new(190, 190, 210, 255));
    }
}
