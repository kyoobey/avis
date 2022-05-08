


use cgmath;



pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
	1.0, 0.0, 0.0, 0.0,
	0.0, 1.0, 0.0, 0.0,
	0.0, 0.0, 0.5, 0.0,
	0.0, 0.0, 0.5, 1.0,
);



const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;



#[derive(Debug)]
pub struct Camera {
	pub position: Point3<f32>,
	pub yaw: Rad<f32>,
	pub pitch: Rad<f32>,
}

impl Camera {
	pub fn new(
		position: Into<cgmath::Vector3<f32>>,
		yaw: Into<Rad<f32>>,
		pitch: Into<Rad<f32>>
	) -> Self {
		Self {
			position: position.into(),
			yaw: yaw.into(),
			pitch: pitch.into()
		}
	}
}


