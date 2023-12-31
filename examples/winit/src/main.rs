use futures::executor::block_on;
use g2d::{Dimension, Handle, Texture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let mut handle = g2d::WindowHandle::new(&window, g2d::Dimension::new(1280, 720))
        .await
        .unwrap();

    // Set visible here to stop flashing white when it starts
    window.set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        control_flow.set_poll();

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                handle.resize_surface(Dimension::new(physical_size.width, physical_size.height));
            }
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
                ..
            } => {
                handle.resize_surface(Dimension::new(new_inner_size.width, new_inner_size.height));
            }

            Event::MainEventsCleared => {
                // Application update code.

                let triangle_buffer = handle.make_vertex_buffer(&[
                    g2d::Vertex::new(
                        g2d::Vec2::new(0.0, 0.0),
                        g2d::Vec2::default(),
                        g2d::Color::WHITE,
                    ),
                    g2d::Vertex::new(
                        g2d::Vec2::new(-0.5, -0.5),
                        g2d::Vec2::default(),
                        g2d::Color::BLACK,
                    ),
                    g2d::Vertex::new(
                        g2d::Vec2::new(0.5, -0.5),
                        g2d::Vec2::default(),
                        g2d::Color::BLACK,
                    ),
                ]);

                // Render frame
                let current_frame = handle.frame().unwrap();
                current_frame.canvas().clear(g2d::Color::BLACK).unwrap();
                current_frame
                    .canvas()
                    .draw_vertices(&triangle_buffer, g2d::Paint::Fill)
                    .unwrap();

                current_frame.present();
                println!("FRAME");
            }

            _ => (),
        }
    });
}

fn main() {
    block_on(run());
}
