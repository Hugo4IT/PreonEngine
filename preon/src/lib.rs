use preon_core::{PreonCore, PreonRenderer};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};

pub struct Preon<T: PreonRenderer> {
    core: Option<PreonCore>,
    renderer: Option<T>,
    window: Option<Window>,
    event_loop: Option<EventLoop<()>>
}

pub fn init<T: PreonRenderer>() -> Preon<T> {
    let (window, event_loop) = {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        (window, event_loop)
    };

    Preon {
        core: Some(PreonCore::init()),
        renderer: Some(T::init()),
        window: Some(window),
        event_loop: Some(event_loop)
    }
}

pub fn start<T: PreonRenderer>(engine: &mut Preon<T>) {
    let core = engine.core.take().unwrap();
    let mut renderer = engine.renderer.take().unwrap();
    let window = engine.window.take().unwrap();
    let handler = engine.event_loop.take().unwrap();

    renderer.start(&core);

    handler.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}

pub fn update<T: PreonRenderer>(core: &mut PreonCore, renderer: &mut T) -> bool {
    core.update();
    renderer.update(core)
}

#[inline(always)]
pub fn render<T: PreonRenderer>(core: &mut PreonCore, renderer: &mut T) {
    renderer.render(core);
}