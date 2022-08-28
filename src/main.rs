use x11;

fn main() {
    println!("Hello, world!");

    move_mouse();
}

// Ref. https://docs.rs/x11/latest/x11/xinput2/fn.XIWarpPointer.html
fn move_mouse() {

    unsafe {
        x11::xinput2::XIWarpPointer(0, 0, 0, 5, 5.0, 5.0, 4, 3, 2.0, 1.0);
    }
}