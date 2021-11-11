use arrayvec::ArrayString;
use super::residue::Residue;
use std::ops::Index;

//mod residue;


pub struct Chain {
	residues: Vec<Residue>,
	chainid : ArrayString< 1 >,
}

impl Chain {

	pub fn new() -> Self {
		Self {
			residues: Vec::<Residue>::new(),
			chainid: ArrayString::< 1 >::new(),
		}
	}
	pub fn residues(&self)            -> &Vec<Residue> { &self.residues }
	pub fn residues_as_mut(&mut self) -> &mut Vec<Residue> { &mut self.residues }
	pub fn chain_id(&self)            -> ArrayString< 1 > { self.chainid }
	pub fn chain_id_as_mut(&mut self) -> &mut ArrayString< 1 > { &mut self.chainid }
	pub fn chain_id_as_str(&self)     -> &str { &self.chainid }
	pub fn len(&self)                 -> usize { self.residues.len() }

}

impl Index<usize> for Chain {
	type Output = Residue;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.residues[idx]
	}
}

//impl Index<&str> for Chain {
//	type Output = Vec<Residue>;
//
//	fn index(&self, resiname: &str) -> &Self::Output {
//		self.residues.iter().filter(|residue| residue.residue_name_as_str() == resiname).collect()
//	}
//}
