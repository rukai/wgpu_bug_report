use winit::{event_loop::EventLoop, window::Window};

pub async fn run(element: web_sys::Element) {
    use winit::platform::web::WindowExtWebSys;

    let event_loop = EventLoop::new().unwrap();
    let window = Window::new(&event_loop).unwrap();

    let canvas = window.canvas().unwrap();
    canvas
        .style()
        .set_css_text("display: block; width: 100%; height: 100%");

    element
        .append_child(&web_sys::Element::from(canvas))
        .unwrap();

    let instance = wgpu::Instance::default();

    let surface = instance.create_surface(&window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");
    log::warn!("succesfully created adapter {adapter:?}");
}

pub async fn render_window_wasm() {
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlElement;

    let document = web_sys::window().unwrap().document().unwrap();

    let body = document.body().unwrap();
    let parent_div = document.create_element("div").unwrap();
    parent_div
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .style()
        .set_css_text("margin: auto; width: 80%; aspect-ratio: 4 / 2; background-color: black");
    body.append_child(&parent_div).unwrap();

    run(parent_div).await;
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("could not initialize logger");

    wasm_bindgen_futures::spawn_local(render_window_wasm());
}
