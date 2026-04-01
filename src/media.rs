use raylib::prelude::*;

pub struct MediaBrowser {
    pub items: Vec<String>,
    pub selected: Option<usize>,
    pub scroll_offset: i32,
    pub add_clicked: bool,
    pub dragging_item: Option<String>,
}

impl MediaBrowser {
    pub fn new() -> Self {
        Self { items: Vec::new(), selected: None, scroll_offset: 0, add_clicked: false, dragging_item: None }
    }

    pub fn add(&mut self, path: String) {
        if !self.items.contains(&path) {
            self.items.push(path);
            self.selected = Some(self.items.len() - 1);
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.selected = None;
        self.scroll_offset = 0;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, x: i32, y: i32, w: i32, h: i32, mouse: Vector2, lmb_down: bool, lmb_click: bool) {
        self.add_clicked = false;

        // Panel border and background
        d.draw_rectangle(x, y, w, h, Color::new(33, 34, 40, 255));
        d.draw_rectangle_lines(x, y, w, h, Color::new(85, 85, 100, 255));

        // Add button at top
        let add_btn_w = w - 16;
        let add_btn_h = 28;
        let add_btn_x = x + 8;
        let add_btn_y = y + 8;

        let is_over_add = mouse.x >= add_btn_x as f32 && mouse.x <= (add_btn_x + add_btn_w) as f32
            && mouse.y >= add_btn_y as f32 && mouse.y <= (add_btn_y + add_btn_h) as f32;

        let add_bg = if is_over_add && lmb_down { Color::new(58, 120, 210, 255) }
                     else if is_over_add { Color::new(68, 130, 220, 255) }
                     else { Color::new(56, 66, 76, 255) };

        d.draw_rectangle(add_btn_x, add_btn_y, add_btn_w, add_btn_h, add_bg);
        d.draw_rectangle_lines(add_btn_x, add_btn_y, add_btn_w, add_btn_h, Color::new(110, 125, 145, 255));
        d.draw_text("+ Add Media (Drop files onto window)", add_btn_x + 8, add_btn_y + 7, 14, Color::WHITE);

        if is_over_add && lmb_click {
            self.add_clicked = true;
        }

        // List title
        d.draw_text("Media Browser", x + 10, add_btn_y + add_btn_h + 8, 16, Color::WHITE);

        // List area
        let content_y = add_btn_y + add_btn_h + 36;
        let content_h = h - (content_y - y) - 12;
        let item_h = 26;

        // scroll with mouse wheel
        let wheel = d.get_mouse_wheel_move();
        self.scroll_offset += (wheel * 24.0) as i32;
        let max_scroll = ((self.items.len() as i32) * item_h - content_h).max(0);
        self.scroll_offset = self.scroll_offset.clamp(0, max_scroll);

        let mut y_cursor = content_y - self.scroll_offset;
        for (i, item) in self.items.iter().enumerate() {
            if y_cursor + item_h < content_y { y_cursor += item_h; continue; }
            if y_cursor > content_y + content_h { break; }

            let item_rect = Rectangle { x: x as f32 + 4.0, y: y_cursor as f32, width: (w - 8) as f32, height: item_h as f32 };
            let hovered = mouse.x >= item_rect.x && mouse.x <= item_rect.x + item_rect.width
                && mouse.y >= item_rect.y && mouse.y <= item_rect.y + item_rect.height;

            let bg = if Some(i) == self.selected { Color::new(70, 90, 145, 230) }
                     else if hovered { Color::new(45, 55, 70, 255) }
                     else { Color::new(0, 0, 0, 0) };

            if bg.a != 0 {
                d.draw_rectangle(item_rect.x as i32, item_rect.y as i32, item_rect.width as i32, item_rect.height as i32, bg);
            }

            // Filename
            let fname = std::path::Path::new(item).file_name().and_then(|n| n.to_str()).unwrap_or("");
            d.draw_text(&fname, item_rect.x as i32 + 6, item_rect.y as i32 + 6, 14, Color::WHITE);

            if hovered && lmb_click {
                self.selected = Some(i);
            }

            // Start dragging
            if hovered && lmb_down && d.get_mouse_delta().length() > 2.0 {
                if self.dragging_item.is_none() {
                    self.dragging_item = Some(item.clone());
                }
            }

            y_cursor += item_h;
        }

        if !lmb_down && !lmb_click {
            self.dragging_item = None;
        }
    }
}

