use std::convert::TryInto;
use crate::math::coordinates::Coordinates;

pub struct DCDTraj {

	signature         : [u8; 4],
	frame_num         : i32,
	initial_frame_id  : i32,
	frame_interval_num: i32,
	step_num          : i32,
	unit_num          : i32,
	time_step         : f32,
	version           : i32,
	atom_num          : i32,
	trajectory        : Vec<Coordinates<f32>>,
}



impl DCDTraj {

	pub fn new(header_1st_block: &[u8], atom_num: i32, trajectory: Vec<Coordinates<f32>>) -> Self {
		Self {
			signature         : header_1st_block[0..4].try_into().expect("Can't convert [u8] to [u8;4] for parsing 'signature'"),
			frame_num         : i32::from_le_bytes(header_1st_block[4..8].try_into().expect("Can't convert [u8] to [u8;4] for parsing 'frame_num'")),
			initial_frame_id  : i32::from_le_bytes(header_1st_block[8..12].try_into().expect("Can't convert [u8] to [u8;4] for parsing 'initial_frame_id'")),
			frame_interval_num: i32::from_le_bytes(header_1st_block[12..16].try_into().expect("Can't convert [u8] to [u8;4] for parsing 'frame_interval_num'")),
			step_num          : i32::from_le_bytes(header_1st_block[16..20].try_into().expect("Can't convert [u8] to [u8;4] for parsing 'step_num'")),
			unit_num          : i32::from_le_bytes(header_1st_block[20..24].try_into().expect("Can't convert [u8] to [u8;4] for parsing 'unit_num'")),
			time_step         : f32::from_le_bytes(header_1st_block[40..44].try_into().expect("Can't convert [u8] to [u8;4] for parsing 'time_step'")),
			version           : i32::from_le_bytes(header_1st_block[80..84].try_into().expect("Can't convert [u8] to [u8;4] for parsing 'version'")),
			atom_num,
			trajectory,
		}
	}

	pub fn signature(&self) -> Result<&str, String> {
		match std::str::from_utf8(&self.signature) {
			Ok(val) => Ok(val),
			Err(err) => Err(err.to_string()),
		}
	}
	pub fn frame_num(&self)             -> i32 {self.frame_num}
	pub fn initial_frame_id(&self)      -> i32 {self.initial_frame_id}
	pub fn frame_interval_num(&self)    -> i32 {self.frame_interval_num}
	pub fn step_num(&self)              -> i32 {self.step_num}
	pub fn unit_num(&self)              -> i32 {self.unit_num}
	pub fn time_step(&self)             -> f32 {self.time_step}
	pub fn version(&self)               -> i32 {self.version}
	pub fn atom_num(&self)              -> i32 {self.atom_num}
	pub fn trajectory(&self)            -> &Vec<Coordinates<f32>> {&self.trajectory}
	pub fn trajectory_as_mut(&mut self) -> &mut Vec<Coordinates<f32>> {&mut self.trajectory}
	pub fn change_trajectory(&mut self, traj: Vec<Coordinates<f32>>) {self.trajectory = traj}
	pub fn change_init_frame(&mut self, ini_frame: i32) {self.initial_frame_id = ini_frame}
	pub fn change_frame_num(&mut self, frame_number: i32) {self.frame_num = frame_number}
	pub fn change_step_num(&mut self, step_number: i32) {self.step_num = step_number}
}
