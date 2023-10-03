use super::residue::Residue;
use std::ops::Index;

//mod residue;


pub struct Chain {
	residues: Vec<Residue>,
	chainid : usize,
}

impl Chain {

	pub fn new(chainid: usize) -> Self {
		Self {
			residues: Vec::<Residue>::new(),
			chainid,
		}
	}

	pub fn residues(&self)            -> &Vec<Residue> { &self.residues }
	pub fn residues_as_mut(&mut self) -> &mut Vec<Residue> { &mut self.residues }
	pub fn residue_ids(&self)         -> Vec<i32> { self.residues.iter().map(|res| res.residue_id()).collect() }
	pub fn chain_id(&self)            -> &usize { &self.chainid }
	pub fn len(&self)                 -> usize { self.residues.len() }

	pub fn change_chain_id(&mut self, chain_id: usize) {self.chainid = chain_id;}

}

impl Index<usize> for Chain {
	type Output = Residue;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.residues[idx]
	}
}

