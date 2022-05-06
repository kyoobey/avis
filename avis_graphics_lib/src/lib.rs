


#[macro_use]
extern crate log;

use wgpu;
use winit::{
	event::WindowEvent,
	event_loop::{ ControlFlow, EventLoop },
	window::{ WindowBuilder, Window, Fullscreen }
};



pub fn get_avis_graphics_lib_version() -> &'static str {
	"v0.0.0"
}



pub fn setup_window_and_event_loop() -> (EventLoop<()>, Window) {
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build(&event_loop)
		.expect("Couldn't create a window");
	trace!("Created window");

	(event_loop, window)
}


pub struct GraphicsState {
	//
}

impl GraphicsState {

	pub async fn new(window: &Window) -> Self {
		let instance = wgpu::Instance::new(wgpu::Backends::all());
		let surface = unsafe { instance.create_surface(&window) };
		trace!("Created surface to render on");

		trace!("initializing wgpu");
		let adapter = instance.request_adapter(
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::LowPower,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false
			}
		).await.expect("Couldn't find display adapter");
		debug!("Using adapter: {}", adapter.get_info().name);



		Self {}
	}


	pub fn input(&mut self, event: &WindowEvent) -> bool {
		false
	}

}


