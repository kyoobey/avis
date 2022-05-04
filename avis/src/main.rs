

use avis_audio_lib;
use avis_graphics_lib;


fn main() {

	println!("hello from avis");

	println!("Version of Avis audio: {}", avis_audio_lib::get_avis_audio_lib_version());

	println!("Version of Avis graphics: {}", avis_graphics_lib::get_avis_graphics_lib_version());

}

