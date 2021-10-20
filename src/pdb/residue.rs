use arrayvec::ArrayString;
use std::ops::Index;
use super::atom::Atom;

//mod atom;


pub struct Residue {
	atoms: Vec<Atom>,
	name : ArrayString< 3 >,
	resid: i32,
	itr  : usize,
}


impl Residue {

	pub fn new(atoms: Vec<Atom>, name: ArrayString< 3 >, resid: i32) -> Self {
		Self {
			atoms,
			name,
			resid,
			itr: 0,
		}
	}
	pub fn atoms(&self)               -> &Vec<Atom> {&self.atoms}
	pub fn atoms_as_mut(&mut self)    -> &mut Vec<Atom> {&mut self.atoms}
	pub fn residue_name(&self)        -> ArrayString< 3 > {self.name}
	pub fn residue_name_as_str(&self) -> &str {&self.name}
	pub fn residue_id(&self)          -> i32 {self.resid}
	pub fn len(&self)                 -> usize {self.atoms.len()}

}

impl Index<usize> for Residue {
	type Output = Atom;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.atoms[idx]
	}
}

//impl Index<&str> for Residue {
//	type Output = Vec<Atom>;
//
//	fn index(&self, atomname: &str) -> &Self::Output {
//		self.atoms.iter().filter(|atom| atom.atom_name_as_str() == atomname).collect()
//	}
//}
