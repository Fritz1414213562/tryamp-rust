use std::io::Read;
use std::convert::TryInto;
use super::dcd_traj::DCDTraj;
use crate::math::coordinates::Coordinates;
use super::constant::{DEFAULT_BUFFER_SCALE, COORDINATE_BLOCK_OFFSET, COORDINATE_BLOCK_SCALE, UNIT_CELL_OFFSET};


pub struct DCDParser;

impl DCDParser {

	pub fn new() -> Self {
		Self
	}

	pub fn read<R: Read>(&self, mut file: R) -> Result<DCDTraj, String> {

	//	let mut file: BufReader<R> = BufReader::<R>::with_capacity(1_000_000_000, file);

		let header_1st  = self.read_block(&mut file)?;
		let _header_2nd = self.read_block(&mut file)?;
		let header_3rd  = self.read_block(&mut file)?;
	
		let has_unitcell: bool = i32::from_le_bytes(header_1st[44..48].try_into().expect("Can't convert [u8] to [u8;4] for parsing has_unitcell")) != 0;

		let frame_num: i32 = i32::from_le_bytes(header_1st[4..8].try_into().expect("Can't convert [u8] to [u8;4] for parsing 'frame_num'"));
		let atom_num : i32 = i32::from_le_bytes(header_3rd.try_into().expect("Can't convert [u8] to [u8;4] for parsing 'atom_num'"));
		let mut traj: Vec<Coordinates<f32>> = Vec::<Coordinates<f32>>::new();

		let coord_block_size: usize = if has_unitcell {
			COORDINATE_BLOCK_SCALE * (atom_num as usize * std::mem::size_of::<i32>() + COORDINATE_BLOCK_OFFSET) + UNIT_CELL_OFFSET
		} else {
			COORDINATE_BLOCK_SCALE * (atom_num as usize * std::mem::size_of::<i32>() + COORDINATE_BLOCK_OFFSET)
		};
		let buffer_capacity: usize = DEFAULT_BUFFER_SCALE * coord_block_size;
		let mut buffer: Vec<u8> = vec![0; buffer_capacity];

		loop {
			match file.read(&mut buffer) {
				Ok(0) => break,
				Ok(buffer_size) => {
					match buffer[..buffer_size].chunks(coord_block_size).map(|mut xyz_bytes| self.read_xyz(&mut xyz_bytes, has_unitcell)).collect::<Result<Vec<Coordinates<f32>>, String>>() {
						Ok(part_traj) => traj.extend(part_traj),
						Err(err) => return Err(err),
					}
				}
				Err(err) => return Err(err.to_string()),
			}
		}

	//	for _ in 0..frame_num { traj.push(self.read_xyz(&mut file)?); }

		if traj.len() != frame_num as usize {
			eprintln!("Warning: The number of Read frames is inconsistent with that recorded in header. ({} != {})", traj.len(), frame_num);
		}

		Ok(DCDTraj::new(&header_1st, atom_num, traj))
	}


	fn read_xyz<R: Read>(&self, bytes: &mut R, has_unitcell: bool) -> Result<Coordinates<f32>, String> {
		if has_unitcell { let _ = self.read_block(bytes)?; }
		let coord_x_bytes = self.read_block(bytes)?;
		let coord_xs: Vec<f32> = coord_x_bytes.chunks(4).map(|coord_x_byte| f32::from_le_bytes(coord_x_byte.try_into().expect("Can't convert [u8] to [u8; 4] for parsing coordinate_x"))).collect();
		let coord_y_bytes = self.read_block(bytes)?;
		let coord_ys: Vec<f32> = coord_y_bytes.chunks(4).map(|coord_y_byte| f32::from_le_bytes(coord_y_byte.try_into().expect("Can't convert [u8] to [u8; 4] for parsing coordinate_y"))).collect();
		let coord_z_bytes = self.read_block(bytes)?;
		let coord_zs: Vec<f32> = coord_z_bytes.chunks(4).map(|coord_z_byte| f32::from_le_bytes(coord_z_byte.try_into().expect("Can't convert [u8] to [u8; 4] for parsing coordinate_z"))).collect();
		Coordinates::<f32>::new(coord_xs, coord_ys, coord_zs)
	}


	fn read_block<R: Read>(&self, bytes: &mut R) -> Result<Vec<u8>, String> {
		let block_size: i32 = self.read_block_size(bytes)?;
		let mut body: Vec<u8> = vec![0; block_size as usize];
		let _ = match bytes.read(&mut body[..]) {
			Ok(bufsize) => bufsize,
			Err(err) => return Err(err.to_string()),
		};
		let check_block_size: i32 = self.read_block_size(bytes)?;
		if block_size != check_block_size {
			return Err(format!("File block is broken. Former block size, {} is inconsistent with latter one, {}.", block_size, check_block_size).to_string());
		}
		Ok(body)
	}

	fn read_block_size<R: Read>(&self, bytes: &mut R) -> Result<i32, String> {
		let mut block_size: [u8; 4] = Default::default();
		let _ = match bytes.read(&mut block_size) {
			Ok(bufsize) => bufsize,
			Err(err) => return Err(err.to_string()),
		};
		Ok(i32::from_le_bytes(block_size))
	}

}
