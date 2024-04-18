extern crate glfw;

use std::env;

use gl;
use glfw::Context;

mod opengl;

fn main() {
    let path = env::current_dir().unwrap();
    println!("Current Path: {} > ", path.display());

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(
            1280,
            720,
            "Hello this is window",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        clear_color(Color(0.4, 0.4, 0.4, 1.0));
    }

    let mut regarded = opengl::shader::Shader {id: 0};

    regarded.load("data\\shaders\\basic.vert", "data\\shaders\\basic.frag");

    let vertecies = [-0.5f32, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };

    let mut vbo = 0;
    unsafe { gl::GenBuffers(1, &mut vbo) };

    unsafe {
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertecies) as isize,
            vertecies.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // -------------------------------------------
    println!("OpenGL version: {}", gl_get_string(gl::VERSION));
    println!(
        "GLSL version: {}",
        gl_get_string(gl::SHADING_LANGUAGE_VERSION)
    );

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        clear_color(Color(0.8, 0.5, 0.5, 1.0));

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        unsafe {
            regarded.use_as_current();
            //gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::BindVertexArray(0);
        }

        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::Action;
    use glfw::Key;
    use glfw::WindowEvent as Event;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

pub struct Color(f32, f32, f32, f32);

pub fn clear_color(c: Color) {
    unsafe { gl::ClearColor(c.0, c.1, c.2, c.3) }
}

pub fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
    let v = unsafe { gl::GetString(name) };
    let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    v.to_str().unwrap()
}
