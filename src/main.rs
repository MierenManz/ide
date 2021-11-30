mod state;
use pollster::block_on;
use state::State;

use winit::dpi::PhysicalSize;
use winit::dpi::Size;
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

    let mut gpu_state = block_on(State::new(&window));
    gpu_state.render().unwrap();

    event_loop.run(move |event, window, control_flow| {
        *control_flow = ControlFlow::Wait;
    });
}
