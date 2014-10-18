#![feature(globs)] //Enable experimental glob import

extern crate native;
extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::Context;
use std::mem;
use std::ptr;
use std::str;

static VERTEX_DATA: [GLfloat, ..12] = [
   -0.5,  0.5,
    0.5,  0.5,
   -0.5, -0.5,
    0.5,  0.5,
    0.5, -0.5,
   -0.5, -0.5
];

static UV_DATA: [GLfloat, ..12] = [
    0.0,  1.0,
    1.0,  1.0,
    0.0,  0.0,
    1.0,  1.0,
    1.0,  0.0,
    0.0,  0.0
];

static VS_SRC: &'static str =
    "#version 150\n\
    in vec2 position;\n\
    in vec2 vert_tex_coord;
    out vec2 frag_tex_coord;
    void main() { \n\
      frag_tex_coord = vert_tex_coord;
      gl_Position = vec4(position, 0.0, 1.0); \n\
    }";

static FS_SRC: &'static str =
   "#version 150\n\
    uniform sampler2D tex;\n\
    in vec2 tex_coord;\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = texture(tex,tex_coord)\n\
    }";

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader = gl::CreateShader(ty);
    unsafe {
        // Attempt to compile the shader
        src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            fail!("{}", str::from_utf8(buf.as_slice()).expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    unsafe {
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            fail!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
        }
    }
    program
}



#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    println!("Hello, world!")
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.window_hint(glfw::ContextVersion(3,2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
    let (window,events) = glfw.create_window(300,300, "Xfcete | Xfce Theme Editor", glfw::Windowed)
                              .expect("Failed to create GLFW window.");
    
    window.set_key_polling(true);
    window.make_current();
    
    gl::load_with(|s| window.get_proc_address(s));
    
    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;
    
    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTEX_DATA[0]),
                       gl::STATIC_DRAW);
        
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, //TODO ... finish this!

        // Use shader program
        gl::UseProgram(program);
        "out_color".with_c_str(|ptr| gl::BindFragDataLocation(program, 0, ptr));

        // Specify the layout of the vertex data
        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());
    }
    
    while !window.should_close() {
        glfw.poll_events();
        for(_,event) in glfw::flush_messages(&events) {
            handle_window_event(&window,event);
        }
        
        gl::ClearColor(0.1,0.1,0.3,1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        
        // Draw a triangle from the 3 vertices
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        
        
        window.swap_buffers();
    }
    
    // Cleanup
    gl::DeleteProgram(program);
    gl::DeleteShader(fs);
    gl::DeleteShader(vs);
    unsafe {
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}




fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
