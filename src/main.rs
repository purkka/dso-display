use glium::{glutin, Surface, implement_vertex, uniform};
use std::time::{SystemTime};

#[derive(Copy, Clone)]
struct Vertex<T> {
    position: [T; 2],
    tex_coords: [T; 2],
}

impl<T> Vertex<T> {
    fn new((x, y): (T, T), (tx, ty): (T, T)) -> Self {
        Self {
            position: [x, y],
            tex_coords: [tx, ty],
        }
    }
}

fn main() {
    const WIDTH: u32 = 320;
    const HEIGHT: u32 = 234;

    // Variables for opening a window
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("test")
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT));
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let pixel_data = vec![
        0u8, 0u8, 0u8, 0u8,
        255u8, 0u8, 0u8, 55u8,
        0u8, 255u8, 0u8, 155u8,
        0u8, 0u8, 255u8, 255u8,
    ];

    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&pixel_data, (2u32, 2u32));
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    type VertexF32 = Vertex<f32>;
    implement_vertex!(VertexF32, position, tex_coords);

    // Two triangles in a fan shape
    let vertex1 = Vertex::new((1.0, 1.0), (1.0, 1.0));
    let vertex2 = Vertex::new((-1.0, 1.0), (0.0, 1.0));
    let vertex3 = Vertex::new((-1.0, -1.0), (0.0, 0.0));
    let vertex4 = Vertex::new((1.0, -1.0), (1.0, 0.0));
    let shape: Vec<Vertex<f32>> = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Dummy indices for the time being
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let vertex_shader_bg = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0, 1);
        }
    "#;

    let fragment_shader_bg = r#"
        #version 140
        #define PI 3.141592

        out vec4 color;

        uniform float time;
        uniform uint height;

        void main() {
            vec2 uv = gl_FragCoord.xy / height;
            uv.y += time / 8;

            // variables
            float split = 8;
            float ncol = 2;

            // the slope
            uv.y -= tan(PI / 4) * uv.x;

            // colors every other stripe
            if (mod(floor(uv.y * split), ncol) == 0) {
                color = vec4(0.25, 0.25, 0.25, 1);
            } else { // keep transparent
                color = vec4(0);
            }
        }
    "#;

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program_src = glium::Program::from_source(&display, vertex_shader_src,
                                                  fragment_shader_src, None).unwrap();
    let program_bg = glium::Program::from_source(&display, vertex_shader_bg,
                                                 fragment_shader_bg, None).unwrap();

    let draw_parameters = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        ..Default::default()
    };

    let start_time = SystemTime::now();
    // Keep the window open until the user closes it
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => (),
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let uniforms_bg = uniform! {
            height: HEIGHT,
            time: SystemTime::now().duration_since(start_time).unwrap().as_secs_f32(),
        };
        let uniforms_src = uniform! {
            // Use GL_NEAREST as the magnifying filter to get the desired (non-blurred) output
            tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        let mut target = display.draw();
        target.clear_color(0.2, 0.2, 0.2, 1.0);
        target.draw(&vertex_buffer, &indices, &program_bg, &uniforms_bg,
                    &draw_parameters).unwrap();
        target.draw(&vertex_buffer, &indices, &program_src, &uniforms_src,
                    &draw_parameters).unwrap();
        target.finish().unwrap();
    });
}
