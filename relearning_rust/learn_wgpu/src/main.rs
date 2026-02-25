use glfw::{Action, Context, Key, fail_on_errors};

fn main() {
    
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw.create_window(
        800, 600, "Its WGPU time", glfw::WindowMode::Windowed).unwrap();

        window.set_key_polling(true);

    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _,Action::Press ,_ ) => {
                    window.set_should_close(true);
                }

                e => {
                    println!("{:?}", e)
                }
            }
        }
        window.swap_buffers();
    }
}
