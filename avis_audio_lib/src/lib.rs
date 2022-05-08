


#[macro_use]
extern crate log;

use std::sync::Once;

use cpal;
use cpal::traits::HostTrait;
use cpal::traits::DeviceTrait;
use crossbeam_channel::Sender;



pub fn get_avis_audio_lib_version() -> &'static str {
	"v0.0.0"
}



pub fn get_default_audio_output_device() -> Option<cpal::Device> {
	let default_host = cpal::default_host();
	default_host.default_output_device()
}



pub fn get_audio_device_name(
	device: &cpal::Device
) -> Option<String> {
	match device.name() {
		Ok(s) => Some(s),
		Err(_) => None
	}
}



pub fn get_audio_device_config(
	device: &cpal::Device
) -> Option<cpal::SupportedStreamConfig> {
	match device.default_output_config() {
		Ok(s) => Some(s),
		Err(_) => None
	}
}



pub fn capture_output_stream(
	device: &cpal::Device,
	audio_data_channel: Sender<Vec<f32>>,
	f32_samples_capacity: u16
) -> Option<cpal::Stream> {

	let audio_config = device.default_output_config().unwrap();

	let mut f32_samples: Vec<f32> = Vec::with_capacity(f32_samples_capacity.into());
	match audio_config.sample_format() {
		cpal::SampleFormat::F32 => device.build_input_stream(
			&audio_config.into(),
			move |data, _: &_| wave_reader::<f32>(data, &mut f32_samples, audio_data_channel.clone()),
			audio_capture_error_function
		).ok(),
		cpal::SampleFormat::I16 => device.build_input_stream(
			&audio_config.into(),
			move |data, _: &_| wave_reader::<i16>(data, &mut f32_samples, audio_data_channel.clone()),
			audio_capture_error_function
		).ok(),
		cpal::SampleFormat::U16 => device.build_input_stream(
			&audio_config.into(),
			move |data, _: &_| wave_reader::<u16>(data, &mut f32_samples, audio_data_channel.clone()),
			audio_capture_error_function
		).ok()
	}

}



pub fn audio_capture_error_function(err: cpal::StreamError) {
	error!("Error building audio input stream");
	error!("{:?}", err);
}



pub fn wave_reader<T>(samples: &[T], f32_samples: &mut Vec<f32>, audio_data_channel: Sender<Vec<f32>>)
where
	T: cpal::Sample
{
	static INITIALIZER: Once = Once::new();
	INITIALIZER.call_once(|| {
		warn!("The wave_reader is now recieving samples ...");
	});
	f32_samples.clear();
	f32_samples.extend(samples.iter().map(|x| x.to_f32()));
	audio_data_channel.send(f32_samples.to_vec()).unwrap_or_else(|_| {
		error!("couldn't get valid audio data");
	})
}


