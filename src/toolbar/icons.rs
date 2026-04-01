use raylib::prelude::*;
use crate::theme::C_ICON;

/// New — blank page with folded top-right corner
pub fn icon_new(d: &mut RaylibDrawHandle, cx: i32, cy: i32) {
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
pub fn icon_open(d: &mut RaylibDrawHandle, cx: i32, cy: i32) {
    let x = cx - 9;
    let y = cy - 6;
    let w = 18i32;
    let h = 13i32;

    let fc = Color { r: 220, g: 175, b: 65,  a: 255 };
    let hi = Color { r: 242, g: 202, b: 100, a: 255 };
    let sh = Color { r: 183, g: 138, b: 42,  a: 255 };

    // Main body + tab
    d.draw_rectangle(x,     y + 4,     w,     h - 4, fc);
    d.draw_rectangle(x,     y,         9,     5,     fc);
    // Top highlight strip
    d.draw_rectangle(x,     y + 4,     w,     3,     hi);
    // Bottom shadow
    d.draw_rectangle(x,     y + h - 2, w,     2,     sh);
    // Inner gleam (thin line)
    d.draw_rectangle(x + 2, y + 5,     w - 4, 1,     hi);
}

/// Save — classic floppy disk
pub fn icon_save(d: &mut RaylibDrawHandle, cx: i32, cy: i32) {
    let x      = cx - 9;
    let y      = cy - 9;
    let s      = 18i32;
    let slot_h = 7i32;

    // Outer casing
    d.draw_rectangle(x, y, s, s, C_ICON);

    // Disk slot (dark cutout at top)
    d.draw_rectangle(x + 2,     y,          s - 7, slot_h,
        Color { r: 48, g: 48, b: 56, a: 255 });

    // Metal shutter (right side of slot)
    d.draw_rectangle(x + s - 5, y,          3, slot_h,
        Color { r: 175, g: 175, b: 182, a: 255 });

    // Label area (white block, bottom)
    d.draw_rectangle(x + 2, y + slot_h + 2, s - 4, s - slot_h - 4,
        Color { r: 238, g: 238, b: 242, a: 255 });

    // Label decoration lines
    let lc = Color { r: 120, g: 120, b: 180, a: 210 };
    d.draw_rectangle(x + 4, y + slot_h + 4, s - 8,  2, lc);
    d.draw_rectangle(x + 4, y + slot_h + 8, s - 10, 2, lc);
}
