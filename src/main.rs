mod vertex;

use glium::{glutin, implement_vertex, uniform, Surface};
use std::time::SystemTime;

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

    type VertexF32 = vertex::Vertex<f32>;
    implement_vertex!(VertexF32, position, tex_coords);

    // Two triangles in a fan shape
    let vertex1 = vertex::Vertex::new((1.0, 1.0), (1.0, 1.0));
    let vertex2 = vertex::Vertex::new((-1.0, 1.0), (0.0, 1.0));
    let vertex3 = vertex::Vertex::new((-1.0, -1.0), (0.0, 0.0));
    let vertex4 = vertex::Vertex::new((1.0, -1.0), (1.0, 0.0));
    let shape: Vec<VertexF32> = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let dummy_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let program_src = glium::Program::from_source(
        &display,
        include_str!("../shaders/src_vertex.glsl"),
        include_str!("../shaders/src_fragment.glsl"),
        None,
    )
    .unwrap();
    let program_bg = glium::Program::from_source(
        &display,
        include_str!("../shaders/bg_vertex.glsl"),
        include_str!("../shaders/bg_fragment.glsl"),
        None,
    )
    .unwrap();

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

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
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
        target
            .draw(
                &vertex_buffer,
                &dummy_indices,
                &program_bg,
                &uniforms_bg,
                &draw_parameters,
            )
            .unwrap();
        target
            .draw(
                &vertex_buffer,
                &dummy_indices,
                &program_src,
                &uniforms_src,
                &draw_parameters,
            )
            .unwrap();
        target.finish().unwrap();
    });
}
