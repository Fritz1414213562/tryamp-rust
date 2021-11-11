use super::pdbmodel::PDBModel;
use std::ops::Index;
use std::collections::HashMap;


pub struct PDBSystem {
	models: Vec<PDBModel>,
	missing_residues: HashMap<String, Vec<i32>>,
	sequences: HashMap<String, Vec<String>>,
}


impl PDBSystem {
	pub fn new() -> Self {
		Self {
			models: Vec::<PDBModel>::new(),
			missing_residues: HashMap::<String, Vec<i32>>::new(),
			sequences: HashMap::<String, Vec<String>>::new(),
		}
	}

	pub fn models(&self)                      -> &Vec<PDBModel> { &self.models }
	pub fn models_as_mut(&mut self)           -> &mut Vec<PDBModel> { &mut self.models }
	pub fn len(&self)                         -> usize { self.models.len() }
	pub fn missing_residues(&self)            -> &HashMap<String, Vec<i32>> { &self.missing_residues }
	pub fn missing_residues_as_mut(&mut self) -> &mut HashMap<String, Vec<i32>> { &mut self.missing_residues }
	pub fn sequences(&self)                   -> &HashMap<String, Vec<String>> { &self.sequences }
	pub fn sequences_as_mut(&mut self)        -> &mut HashMap<String, Vec<String>> { &mut self.sequences }
}

impl Index<usize> for PDBSystem {
	type Output = PDBModel;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.models[idx]
	}
}
