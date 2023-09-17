pub mod renderer;
pub mod debugging;
pub mod shader;

use crate::shader::{ShaderPart, ShaderProgram};
use crate::renderer::{QuadProps, Renderer};

use glfw::{Key, WindowEvent};
use rand::Rng;
use std::sync::mpsc::Receiver;
use glfw::{Context, ffi::{glfwSwapInterval, glfwGetTime}};
use std::ffi::CString;

#[derive(Default)]

// To check framerate
pub struct Framerate{
    pub frame_count: u32,
    pub last_fram_time: f64,
}

impl Framerate{
    fn run(&mut self){
        self.frame_count += 1;

        let current_time = unsafe {glfwGetTime()};
        let delta = current_time - self.last_fram_time;

        if delta >= 1.0{
            self.last_fram_time = current_time;
            println!("FPS: {}", f64::from(self.frame_count) / delta);
            self.frame_count = 0;
        }
    }
    
}

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

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _); // Load OpenGL function pointers
    unsafe {glfwSwapInterval(0)}; // `1` means Vsync
    // Vsync : 60fps , we can't see more than 60fps on monitor
    // Vsync = 1 : 60fps, Vsync = 2 : 30fps, Vsync = 3 : 20fps

    let mut renderer = Renderer::new(100000);

    let vert = ShaderPart::from_vert_source(&CString::new(include_str!("shaders/vert.vert")).unwrap()).unwrap();
    let frag = ShaderPart::from_frag_source(&CString::new(include_str!("shaders/frag.frag")).unwrap()).unwrap();
    let program = ShaderProgram::from_shaders(vert, frag).unwrap();

    program.use_program();

    let mut framerate = Framerate{
        frame_count: 0,
        last_fram_time: 0.0,
    };

    let mut quads = Vec::new();
    let mut rng = rand::thread_rng();

    while !window.should_close() {
        // Poll and process events
        glfw.poll_events(); // check for events

        // Clear the screen to black

        for (_, event) in glfw::flush_messages(&events) {
            match event{
                glfw::WindowEvent::Key(Key::Space, _, _, _) =>{
                    quads.push(QuadProps{
                        position: (rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                        size: (rng.gen_range(0.1..0.5), rng.gen_range(0.1..0.5)),
                        color: (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 1.0),
                    })
                }
                _ => {}
            }
        }

        gl_call!(gl::ClearColor(1.0, 1.0, 1.0, 1.0)); // RGBA
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT)); // clear color buffer

        renderer.begin_batch();

        for quad in &quads{
            renderer.submit_quad(quad.clone());
        }

        renderer.end_batch();

        // Swap front and back buffers
        // window.swap_buffers();
        
        // double buffering : front buffer, back buffer
        window.swap_buffers(); // swap front and back buffers

        framerate.run();
    }
}

// 준비물
// 1. CMake