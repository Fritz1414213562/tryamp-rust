use super::residue::Residue;
use std::ops::Index;

pub struct PQRModel {
	residues: Vec<Residue>,
}

impl PQRModel {
	pub fn new() -> Self {
		Self {
			residues: Vec::<Residue>::new(),
		}
	}

	pub fn residues(&self)                  -> &Vec<Residue> { &self.residues }
	pub fn residues_as_mut(&mut self)       -> &mut Vec<Residue> { &mut self.residues }
	pub fn residue_of(&self, entry_id: i32) -> Option<&Residue> {
		self.residues.iter().find(|ch| ch.residue_id() == entry_id)
	}
	pub fn residue_ids(&self)               -> Vec<i32> { self.residues.iter().map(|ch| ch.residue_id().clone()).collect() }
	pub fn residue_names(&self)             -> Vec<String> { self.residues.iter().map(|ch| ch.residue_name().to_string()).collect() }
	pub fn len(&self)                       -> usize { self.residues.len() }
}

impl Index<usize> for PQRModel {
	type Output = Residue;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.residues[idx]
	}
}

