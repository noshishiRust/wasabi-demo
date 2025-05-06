use crate::efi::bitmap::BitMap;
use crate::efi::protocol::locate_graphic_protocol;
use crate::efi::table::EfiSystemTable;
use crate::Result;
use core::fmt;

pub fn init_vram(efi_system_table: &EfiSystemTable) -> Result<VramBufferInfo> {
    let gp = locate_graphic_protocol(efi_system_table)?;
    Ok(VramBufferInfo {
        buf: gp.mode.frame_buffer_base as *mut u8,
        width: gp.mode.info.horizontal_resolution as i64,
        height: gp.mode.info.vertical_resolution as i64,
        pixels_per_line: gp.mode.info.pixels_per_scan_line as i64,
    })
}

pub enum Color {
    Black = 0x000000,
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
    Yellow = 0xffff00,
    Cyan = 0x00ffff,
    Magenta = 0xff00ff,
    White = 0xffffff,
}

#[derive(Clone, Copy)]
pub struct VramBufferInfo {
    buf: *mut u8,
    width: i64,
    height: i64,
    pixels_per_line: i64,
}

impl BitMap for VramBufferInfo {
    fn byte_by_pixel(&self) -> i64 {
        4
    }

    fn pixels_per_line(&self) -> i64 {
        self.pixels_per_line
    }

    fn width(&self) -> i64 {
        self.width
    }

    fn height(&self) -> i64 {
        self.height
    }

    fn buf_mut(&mut self) -> *mut u8 {
        self.buf
    }
}

pub struct VramTextWriter<'a> {
    vram: &'a mut VramBufferInfo,
    cursor_x: i64,
    cursor_y: i64,
}

impl<'a> VramTextWriter<'a> {
    pub fn new(vram: &'a mut VramBufferInfo) -> Self {
        Self {
            vram,
            cursor_x: 0,
            cursor_y: 0,
        }
    }
}

impl fmt::Write for VramTextWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if c == '\n' {
                self.cursor_y += 16;
                self.cursor_x = 0;
                continue;
            }
            draw_font_fg(self.vram, self.cursor_x, self.cursor_y, 0xffffff, c);
            self.cursor_x += 8;
        }
        Ok(())
    }
}

/// # Safety
///
/// (x, y) must be a valid point in the buf.
unsafe fn unchecked_draw_point<T: BitMap>(buf: &mut T, color: u32, x: i64, y: i64) {
    *buf.unchecked_pixel_at_mut(x, y) = color;
}

fn draw_point<T: BitMap>(buf: &mut T, color: u32, x: i64, y: i64) -> Result<()> {
    *(buf.pixel_at_mut(x, y).ok_or("Out of Range")?) = color;
    Ok(())
}

/// Calculates a y-coordinate for drawing a line using Bresenham's midpoint algorithm
///
/// This function is used for line drawing when the line is more horizontal than vertical (|dx| >= |dy|).
/// It calculates the y-coordinate for each x position along the line.
///
/// # Arguments
///
/// * `dw` - The width difference (dx) between start and end points
/// * `dh` - The height difference (dy) between start and end points
/// * `x` - The current x position being calculated
///
/// # Returns
///
/// * `Some(y)` - The calculated y-coordinate if the x is within valid range
/// * `None` - If the slope is too steep (|dx| < |dy|) or x is out of range
fn calc_slope_point(dw: i64, dh: i64, x: i64) -> Option<i64> {
    if dw < dh {
        None
    } else if dw == 0 {
        Some(0)
    } else if (0..=dw).contains(&x) {
        Some((2 * dh * x + dw) / dw / 2)
    } else {
        None
    }
}

pub fn draw_line<T: BitMap>(
    buf: &mut T,
    color: u32,
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
) -> Result<()> {
    if !buf.is_in_x_range(x0)
        || !buf.is_in_x_range(x1)
        || !buf.is_in_y_range(y0)
        || !buf.is_in_y_range(y1)
    {
        return Err("Out of Range");
    }
    let dx = (x1 - x0).abs();
    let sx = (x1 - x0).signum();
    let dy = (y1 - y0).abs();
    let sy = (y1 - y0).signum();
    if dx >= dy {
        for (rx, ry) in (0..dx).flat_map(|rx| calc_slope_point(dx, dy, rx).map(|ry| (rx, ry))) {
            draw_point(buf, color, x0 + rx * sx, y0 + ry * sy)?;
        }
    } else {
        for (rx, ry) in (0..dy).flat_map(|ry| calc_slope_point(dy, dx, ry).map(|rx| (rx, ry))) {
            draw_point(buf, color, x0 + rx * sx, y0 + ry * sy)?;
        }
    }
    Ok(())
}

fn draw_font_fg<T: BitMap>(buf: &mut T, x: i64, y: i64, color: u32, c: char) {
    if let Some(font) = lookup_font(c) {
        for (dy, row) in font.iter().enumerate() {
            for (dx, pixel) in row.iter().enumerate() {
                let color = match pixel {
                    '*' => color,
                    _ => continue,
                };
                let _ = draw_point(buf, color, x + dx as i64, y + dy as i64);
            }
        }
    }
}

fn draw_str_fg<T: BitMap>(buf: &mut T, x: i64, y: i64, color: u32, s: &str) {
    for (i, c) in s.chars().enumerate() {
        draw_font_fg(buf, x + i as i64 * 8, y, color, c)
    }
}

fn lookup_font(c: char) -> Option<[[char; 8]; 16]> {
    const FONT_SOURCE: &str = include_str!("./font.txt");
    if let Ok(c) = u8::try_from(c) {
        let mut fi = FONT_SOURCE.split('\n');
        while let Some(line) = fi.next() {
            if let Some(line) = line.strip_prefix("0x") {
                if let Ok(idx) = u8::from_str_radix(line, 16) {
                    if idx != c {
                        continue;
                    }
                    let mut font = [['*'; 8]; 16];
                    for (y, line) in fi.clone().take(16).enumerate() {
                        for (x, c) in line.chars().enumerate() {
                            if let Some(e) = font[y].get_mut(x) {
                                *e = c;
                            }
                        }
                    }
                    return Some(font);
                }
            }
        }
    }
    None
}

pub fn fill_rect<T: BitMap>(
    buf: &mut T,
    color: u32,
    px: i64,
    py: i64,
    w: i64,
    h: i64,
) -> Result<()> {
    if !buf.is_in_x_range(px)
        || !buf.is_in_y_range(py)
        || !buf.is_in_x_range(px + w - 1)
        || !buf.is_in_y_range(py + h - 1)
    {
        return Err("Out of Range");
    }
    for y in py..py + h {
        for x in px..px + w {
            unsafe {
                unchecked_draw_point(buf, color, x, y);
            }
        }
    }
    Ok(())
}

pub fn draw_test_pattern<T: BitMap>(buf: &mut T) {
    let w = 128;
    let left = buf.width() - w - 1;
    let colors = [
        Color::Black as u32,
        Color::Red as u32,
        Color::Green as u32,
        Color::Blue as u32,
    ];
    let h = 64;
    for (i, c) in colors.iter().enumerate() {
        let y = i as i64 * h;
        fill_rect(buf, *c, left, y, h, h).expect("fill_rect failed");
        fill_rect(buf, !*c, left + h, y, h, h).expect("fill_rect failed");
    }
    let points = [(0, 0), (0, w), (w, 0), (w, w)];
    for (x0, y0) in points.iter() {
        for (x1, y1) in points.iter() {
            let _ = draw_line(buf, Color::White as u32, left + *x0, *y0, left + *x1, *y1);
        }
    }
    draw_str_fg(
        buf,
        left,
        h * colors.len() as i64,
        Color::Green as u32,
        "0123456789",
    );
    draw_str_fg(
        buf,
        left,
        h * colors.len() as i64 + 16,
        Color::Green as u32,
        "ABCDEF",
    );
}
