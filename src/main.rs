#![no_std]
#![no_main]

use wasabi_demo::efi::bitmap::BitMap;
use wasabi_demo::efi::vram::init_vram;
use wasabi_demo::efi::{EfiHandle, EfiSystemTable};
use wasabi_demo::{hlt, Result};

enum Color {
    Black = 0x000000,
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
    Yellow = 0xffff00,
    Cyan = 0x00ffff,
    Magenta = 0xff00ff,
    White = 0xffffff,
}


/// UEFI entry point
/// 
/// Reference: <https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#uefi-image-entry-point>
#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let mut vram = init_vram(efi_system_table).expect("Failed to initialize VRAM");
    let vw = vram.width();
    let vh = vram.height();
    fill_rect(&mut vram, Color::Black as u32, 0, 0, vw, vh).expect("fill_rect failed");
    fill_rect(&mut vram, Color::Red as u32, 32, 32, 32, 32).expect("fill_rect failed");
    fill_rect(&mut vram, Color::Green as u32, 64, 64, 64, 64).expect("fill_rect failed");
    fill_rect(&mut vram, Color::Blue as u32, 128, 128, 128, 128).expect("fill_rect failed");
    for i in 0..256 {
        let _ = draw_point(&mut vram, 0x010101 * i as u32, i, i);
    }
    let grid_size = 32_i64;
    let rect_size = grid_size * 8;
    for i in (0..=rect_size).step_by(grid_size as usize) {
        let _ = draw_line(&mut vram, Color::Red as u32, 0, i, rect_size, i); // horizontal
        let _ = draw_line(&mut vram, Color::Red as u32, i, 0, i, rect_size); // vertical
    }

    let cx = rect_size / 2;
    let cy = rect_size / 2;

    for i in (0..=rect_size).step_by(grid_size as usize) {
        let _ = draw_line(&mut vram, Color::Yellow as u32, cx, cy, 0, i); // left
        let _ = draw_line(&mut vram, Color::Cyan as u32, cx, cy, i, 0); // top
        let _ = draw_line(&mut vram, Color::Magenta as u32, cx, cy, rect_size, i); // right
        let _ = draw_line(&mut vram, Color::White as u32, cx, cy, i, rect_size); // bottom
    }

    loop {
        hlt();
    }
}

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt();
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

fn draw_line<T: BitMap>(
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
        for (rx, ry) in (0..dx)
            .flat_map(|rx| calc_slope_point(dx, dy, rx).map(|ry| (rx, ry)))
        {
            draw_point(buf, color, x0 + rx * sx, y0 + ry * sy)?;
        }
    } else {
        for (rx, ry) in (0..dy)
            .flat_map(|ry| calc_slope_point(dy, dx, ry).map(|rx| (rx, ry)))
        {
            draw_point(buf, color, x0 + rx * sx, y0 + ry * sy)?;
        }
    }
    Ok(())
}

fn fill_rect<T: BitMap>(buf: &mut T, color: u32, px: i64, py: i64, w: i64, h: i64) -> Result<()> {
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
