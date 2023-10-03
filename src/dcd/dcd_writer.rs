use std::io::{Write, BufWriter};
use super::dcd_traj::DCDTraj;
use super::constant::{DCD_SECOND_BLOCK, DCD_SECOND_BLOCKSIZE};
use crate::math::coordinates::Coordinates;


pub struct DCDWriter<W: Write> {
	inner: BufWriter<W>,
}

impl<W: Write> DCDWriter<W> {

	pub fn new(inner: W) -> Self {
		Self {
			inner: BufWriter::new(inner),
		}
	}

	pub fn write(&mut self, traj: DCDTraj) -> Result<(), String> {
		self.write_1stblock(&traj)?;
		self.write_2ndblock()?;
		self.write_3rdblock(&traj)?;
		self.write_body(&traj)?;
		self.inner.flush().map_err(|err| err.to_string())?;
		Ok(())
	}

//	pub fn write_body_direct(&mut self, traj: &Vec<Coordinates<f32>>, atom_num: i32) -> Result<(), String> {
	pub fn write_body_direct(&mut self, traj: &[Coordinates<f32>], atom_num: i32) -> Result<(), String> {
		let b_block_size = (atom_num * 4_i32).to_le_bytes();
		for coordinate in traj {
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			let _ = coordinate.xs().iter().try_for_each(|x| self.inner.write_all(&x.to_le_bytes()));
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			let _ = coordinate.ys().iter().try_for_each(|y| self.inner.write_all(&y.to_le_bytes()));
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			let _ = coordinate.zs().iter().try_for_each(|z| self.inner.write_all(&z.to_le_bytes()));
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		}
		Ok(())
	}

	fn write_body(&mut self, traj: &DCDTraj) -> Result<(), String> {
		let b_block_size = (traj.atom_num() * 4_i32).to_le_bytes();
		for coordinate in traj.trajectory() {
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			let _ = coordinate.xs().iter().try_for_each(|x| self.inner.write_all(&x.to_le_bytes()));
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			let _ = coordinate.ys().iter().try_for_each(|y| self.inner.write_all(&y.to_le_bytes()));
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
			let _ = coordinate.zs().iter().try_for_each(|z| self.inner.write_all(&z.to_le_bytes()));
			self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		}
		Ok(())
	}

	pub fn write_1stblock_direct(&mut self, frame_num: i32, init_frame_id: i32, frame_intv: i32, step_num: i32, unit_num: i32, time_step: f32, version: i32) -> Result<(), String> {
		let b_block_size = 84_i32.to_le_bytes();
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		let signature: &[u8] = b"CORD";
		self.inner.write_all(signature).map_err(|err| err.to_string())?;
		self.inner.write_all(&frame_num.to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&init_frame_id.to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&frame_intv.to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&step_num.to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&unit_num.to_le_bytes()).map_err(|err| err.to_string())?;
		for _ in 0..4 { self.inner.write_all(&0_i32.to_le_bytes()).map_err(|err| err.to_string())?; }
		self.inner.write_all(&time_step.to_le_bytes()).map_err(|err| err.to_string())?;
		for _ in 0..9 { self.inner.write_all(&0_i32.to_le_bytes()).map_err(|err| err.to_string())?; }
		self.inner.write_all(&version.to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		Ok(())
	}

	fn write_1stblock(&mut self, traj: &DCDTraj) -> Result<(), String> {
		let b_block_size = 84_i32.to_le_bytes();
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		let signature: &[u8] = b"CORD";
		self.inner.write_all(signature).map_err(|err| err.to_string())?;
		self.inner.write_all(&traj.frame_num().to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&traj.initial_frame_id().to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&traj.frame_interval_num().to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&traj.step_num().to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&traj.unit_num().to_le_bytes()).map_err(|err| err.to_string())?;
		for _ in 0..4 { self.inner.write_all(&0_i32.to_le_bytes()).map_err(|err| err.to_string())?; }
		self.inner.write_all(&traj.time_step().to_le_bytes()).map_err(|err| err.to_string())?;
		for _ in 0..9 { self.inner.write_all(&0_i32.to_le_bytes()).map_err(|err| err.to_string())?; }
		self.inner.write_all(&traj.version().to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		Ok(())
	}

	pub fn write_2ndblock_direct(&mut self) -> Result<(), String> {
		let b_block_size = DCD_SECOND_BLOCKSIZE.to_le_bytes();
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		self.inner.write_all(&1_i32.to_le_bytes()).map_err(|err| err.to_string())?;
		//self.inner.write_all(&DCD_SECOND_BLOCK.as_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&DCD_SECOND_BLOCK).map_err(|err| err.to_string())?;
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		Ok(())
	}

	fn write_2ndblock(&mut self) -> Result<(), String> {
		let b_block_size = DCD_SECOND_BLOCKSIZE.to_le_bytes();
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		self.inner.write_all(&1_i32.to_le_bytes()).map_err(|err| err.to_string())?;
		//self.inner.write_all(&DCD_SECOND_BLOCK.as_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&DCD_SECOND_BLOCK).map_err(|err| err.to_string())?;
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		Ok(())
	}

	pub fn write_3rdblock_direct(&mut self, atom_num: i32) -> Result<(), String> {
		let b_block_size = 4_i32.to_le_bytes();
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		self.inner.write_all(&atom_num.to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		Ok(())
	}

	fn write_3rdblock(&mut self, traj: &DCDTraj) -> Result<(), String> {
		let b_block_size = 4_i32.to_le_bytes();
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		self.inner.write_all(&traj.atom_num().to_le_bytes()).map_err(|err| err.to_string())?;
		self.inner.write_all(&b_block_size).map_err(|err| err.to_string())?;
		Ok(())
	}

}
