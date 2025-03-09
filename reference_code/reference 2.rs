use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut window = Window::new(
        "Multiple Buffers in One Window",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    let mut buffer1 = vec![0xFF0000; (WIDTH * HEIGHT) / 2]; // Top half red
    let mut buffer2 = vec![0x0000FF; (WIDTH * HEIGHT) / 2]; // Bottom half blue
    let mut final_buffer = vec![0; WIDTH * HEIGHT];
    final_buffer[..buffer1.len()].copy_from_slice(&buffer1);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Merge buffers (top half = buffer1, bottom half = buffer2)
        final_buffer[buffer1.len()..].copy_from_slice(&buffer2);

        // Update window with final buffer
        window.update_with_buffer(&final_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
