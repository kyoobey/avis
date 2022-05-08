


#[macro_use]
extern crate log;
use simple_logger;

use avis_audio_lib;
use avis_graphics_lib;

use cpal;
use cpal::traits::StreamTrait;
use crossbeam_channel::{unbounded, Receiver, Sender};
use pollster;
use rustfft::{
	FftPlannerSse,
	num_complex::Complex32
};
use winit::{
	event::{ Event, KeyboardInput, WindowEvent, VirtualKeyCode, ElementState },
	event_loop::{ ControlFlow, EventLoop },
	window::Window
};
use wgpu;

use std::time::{ Instant, Duration };
use std::cmp::{ min, max };



const SKIP_SAMPLES: u8 = 3;
const BLUR_SAMPLES: usize = 2;



fn main() {

	simple_logger::SimpleLogger::new()
		.with_level(log::LevelFilter::Debug)
		.init()
		.expect("Couldn't initialize logger :(, there's something seriously wrong with the dependencies");

	warn!("hello from avis");

	warn!("Version of Avis audio: {}", avis_audio_lib::get_avis_audio_lib_version());
	warn!("Version of Avis graphics: {}", avis_graphics_lib::get_avis_graphics_lib_version());



	let audio_output_device = avis_audio_lib::get_default_audio_output_device()
		.expect("Couldn't find a default audio output device to read from! exiting...");

	let audio_output_device_name = avis_audio_lib::get_audio_device_name(&audio_output_device)
		.expect("Found a default audio output device but can't read the name, huh weird");
	warn!("Audio output device name (readable): {}", audio_output_device_name);

	let audio_device_config = avis_audio_lib::get_audio_device_config(&audio_output_device)
		.expect("No default output config found, i don't know what it means tho, it worked on my computer");
	warn!("Audio default Config: {:?}", audio_device_config);



	warn!("trying to capturing system audio...");

	let audio_data_channel: (Sender<Vec<f32>>, Receiver<Vec<f32>>) = unbounded();
	let stream: cpal::Stream;
	match avis_audio_lib::capture_output_stream(&audio_output_device, audio_data_channel.0, 512*32) { // 512 floats
		Some(s) => {
			stream = s;
			stream.play().expect("Couldn't start capturing audio ...");
		},
		None => error!("Couldn't create loopback device :(")
	}



	// let mut planner = FftPlannerSse::new().expect("Couldn't initialize Fft");
	// let fft = planner.plan_fft_forward(4);

	// loop {
	// 	match audio_data_channel.1.try_recv() {
	// 		Err(_) => {},
	// 		Ok(raw_data) => {
	// 			let min = raw_data.iter().copied().reduce(|a, b| a.min(b)).filter(|m| !m.is_nan()).unwrap();
	// 			// println!("{:?}, {:?}", min, data.len());

	// 			let mut buf: Vec<_> = raw_data.iter().map(|n| Complex32::new(*n, 0.0)).collect();
	// 			fft.process(&mut buf);

	// 			println!("{:?}", &buf.iter().map(|n| n.re).collect::<Vec<f32>>());
	// 			break;
	// 		}
	// 	}
	// }


	warn!("creating a window and seting up event loop");
	let (event_loop, window) = avis_graphics_lib::setup_window_and_event_loop();

	warn!("entering infinite loop");
	pollster::block_on(run_infinite_loop(audio_data_channel.1, event_loop, window));

}


async fn run_infinite_loop(
	audio_data_recieving_channel: Receiver<Vec<f32>>,
	event_loop: EventLoop<()>,
	window: Window
) {
	let mut graphics_state = avis_graphics_lib::GraphicsState::new(&window).await;

	// graphics_state.update(Duration::new(0, 0), &Vec::<f32>::new());

	// let mut graphics_state: avis_graphics_lib::GraphicsState;

	let mut fft_buf_size = 4;
	loop {
		if let Ok(data) = audio_data_recieving_channel.try_recv() {
			fft_buf_size = data.len();
			break;
		}
	}
	warn!("{:?}", fft_buf_size);

	let mut planner = FftPlannerSse::new().expect("Couldn't initialize Fft");
	let fft = planner.plan_fft_forward(fft_buf_size);

	let mut last_render_time = Instant::now();

	let mut sample_counter: u8 = 0;

	event_loop.run(move |event, _, control_flow| match event {
		Event::WindowEvent {
			ref event,
			window_id
		} if window_id == window.id() => {
			if !graphics_state.input(event) {
				match event {
					WindowEvent::CloseRequested
					| WindowEvent::KeyboardInput {
						input: KeyboardInput {
							state: ElementState::Pressed,
							virtual_keycode: Some(VirtualKeyCode::Escape),
							..
						},
						..
					} => *control_flow = ControlFlow::Exit,
					_ => {}
				}
			}
		},
		Event::RedrawRequested(window_id) if window_id == window.id() => {
			let now = Instant::now();
			let dt = now - last_render_time;
			last_render_time = now;

			// for _ in (0..SKIP_SAMPLES) {
			// 	let _ = audio_data_recieving_channel.try_recv().ok();
			// }

			match audio_data_recieving_channel.try_recv() {
				Err(_) => {},
				Ok(raw_data) => {
					if sample_counter >= SKIP_SAMPLES {
						// warn!("{:?}", raw_data.len());
						let mut buf: Vec<_> = raw_data.iter().map(|n| Complex32::new(*n, 0.0)).collect();
						fft.process(&mut buf);

						// const blur_constant: f32 = 1./((BLUR_SAMPLES as f32)*2. + 1.);
						let mut data: Vec<f32> = buf.iter().map(|n| n.norm() / (fft_buf_size as f32)).collect();
						// for i in BLUR_SAMPLES..data.len()-BLUR_SAMPLES {
						// 	let mut x = 0.0;
						// 	for j in 0..BLUR_SAMPLES*2+1 {
						// 		x += data[i+j-BLUR_SAMPLES];
						// 	}
						// 	data[i] = x*blur_constant;
						// }
						// data = data.iter()
						// 	.take(fft_buf_size / 2)
						// 	.map(|frequency| frequency.norm())
						// 	.collect();

						graphics_state.update(dt, &data);

						match graphics_state.render() {
							Ok(_) => {},
							Err(wgpu::SurfaceError::Lost) => graphics_state.resize(graphics_state.size),
							Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
							Err(e) => error!("Couldn't draw frame: {:?}", e)
						}
						sample_counter = 0;
					} else {
						sample_counter += 1;
					}
				}
			}

		},
		Event::RedrawEventsCleared => {
			window.request_redraw();
		},
		_ => {}
	});

}


