use x11::xlib;
use std::ptr;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

fn bezier(t: f64, p0: f64, p1: f64, p2: f64) -> f64 {
    (1.0 - t).powi(2) * p0 + 2.0 * (1.0 - t) * t * p1 + t.powi(2) * p2
}

fn generate_bezier_points(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), steps: usize) -> Vec<(i32, i32)> {
    (0..steps)
        .map(|i| {
            let t = i as f64 / (steps as f64 - 1.0);
            (
                bezier(t, p0.0, p1.0, p2.0).round() as i32,
                bezier(t, p0.1, p1.1, p2.1).round() as i32,
            )
        })
        .collect()
}

unsafe fn move_cursor(display: *mut xlib::Display, points: &[(i32, i32)]) {
    for &(x, y) in points {
        xlib::XWarpPointer(display, 0, xlib::XDefaultRootWindow(display), 0, 0, 0, 0, x, y);
        xlib::XFlush(display);  // Send the request to the X server
        sleep(Duration::from_millis(10));  // Adjustable delay
    }
}

fn move_to_position(display: *mut xlib::Display, start: (i32, i32), end: (i32, i32), steps: usize) {
    let control = (
        (start.0 + end.0) / 2 + rand::thread_rng().gen_range(-50..50),
        (start.1 + end.1) / 2 + rand::thread_rng().gen_range(-50..50),
    );

    let points = generate_bezier_points(
        (start.0 as f64, start.1 as f64),
        (control.0 as f64, control.1 as f64),
        (end.0 as f64, end.1 as f64),
        steps,
    );

    unsafe { move_cursor(display, &points) };
}

fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("Cannot open display.");
        }

        move_to_position(display, (100, 100), (800, 600), 100);

        xlib::XCloseDisplay(display);
    }
}
