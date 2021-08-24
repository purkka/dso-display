use glium::{glutin, Surface};

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("test")
        .with_inner_size(glutin::dpi::LogicalSize::new(320, 234));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &el).unwrap();

    // Keep the window open, until the user closes it
    el.run(move |ev, _, control_flow| {
        // Fill the window with a single color
        let mut target = display.draw();
        target.clear_color(0.8, 0.0, 0.1, 1.0);
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
