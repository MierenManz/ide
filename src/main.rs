mod widget;
use pollster::block_on;
use winit::dpi::PhysicalSize;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(PhysicalSize::new(640, 480))
        .build(&event_loop)
        .unwrap();

    let mut window_state = block_on(WindowState::new(&window));
    window_state.render().unwrap();

    event_loop.run(move |general_event, _window, control_flow| {
        *control_flow = ControlFlow::Wait;
    });
}
