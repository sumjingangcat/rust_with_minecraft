use std::sync::mpsc::Receiver;
use glfw::{Context, ffi::glfwSwapInterval};

fn main() { // we will use openGL
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap(); // Init glfw
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6)); // OpenGL version 4.6 // major 4, minor 6
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    )); // OpenGL core profile
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

    let window_size = (500, 500);
    let window_title = "Minecraft";

    let (mut window, events) = glfw.
    create_window(
        window_size.0,
        window_size.1,
        window_title,
        glfw::WindowMode::Windowed,
    )
    .expect("Failed to create GLFW window");

    // Make the window's context current
    window.make_current();
    window.set_all_polling(true);
    window.set_cursor_enter_polling(true);

    // What is context?

    // Context is a state of OpenGL
    // OpenGL is a state machine
    // OpenGL has a lot of states

    unsafe {glfwSwapInterval(1)}; // `1` means Vsync
    // Vsync : 60fps , we can't see more than 60fps on monitor
    // Vsync = 1 : 60fps, Vsync = 2 : 30fps, Vsync = 3 : 20fps

    while !window.should_close() {
        // Poll and process events
        glfw.poll_events(); // check for events

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
        }
        
        // double buffering : front buffer, back buffer
        window.swap_buffers(); // swap front and back buffers
    }
}

// 준비물
// 1. CMake