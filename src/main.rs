use glium::{glutin, Surface, implement_vertex};

fn main() {
    // Variables for opening a window
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("test")
        .with_inner_size(glutin::dpi::LogicalSize::new(320, 234));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &el).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    // A simple triangle shape
    let vertex1 = Vertex { position: [0.0, 0.4] };
    let vertex2 = Vertex { position: [0.6, -0.2] };
    let vertex3 = Vertex { position: [-0.3, -0.3] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Dummy indices for the time being
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.2, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src,
                                              fragment_shader_src, None).unwrap();

    // Keep the window open until the user closes it
    el.run(move |ev, _, control_flow| {
        // Fill the window with a single color
        let mut target = display.draw();
        target.clear_color(0.08, 0.0, 0.24, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
