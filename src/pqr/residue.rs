use arrayvec::ArrayString;
use std::ops::Index;
use super::atom::Atom;


pub struct Residue {
	atoms: Vec<Atom>,
	name : ArrayString< 3 >,
	resid: i32,
}


impl Residue {

	pub fn new() -> Self {
		Self {
			atoms: Vec::<Atom>::new(),
			name: ArrayString::< 3 >::new(),
			resid: 0,
		}
	}
	pub fn atoms(&self)                             -> &Vec<Atom> {&self.atoms}
	pub fn atoms_as_mut(&mut self)                  -> &mut Vec<Atom> {&mut self.atoms}
	pub fn residue_name(&self)                      -> &str {&self.name}
	pub fn residue_id(&self)                        -> i32 {self.resid}
	pub fn len(&self)                               -> usize {self.atoms.len()}

	pub fn change_residue_id(&mut self, resid: i32)    {self.resid = resid;}

	pub fn change_residue_name(&mut self, resin: &str) -> Result<(), String> {
		self.name = ArrayString::<3>::from(resin).map_err(|err| err.to_string())?;
		Ok(())
	}

}

impl Index<usize> for Residue {
	type Output = Atom;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.atoms[idx]
	}
}

