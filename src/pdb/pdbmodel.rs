use super::chain::Chain;
use std::ops::Index;
//mod chain;

pub struct PDBModel {
	chains: Vec<Chain>,
}

impl PDBModel {
	pub fn new() -> Self {
		Self {
			chains: Vec::<Chain>::new(),
		}
	}

	pub fn chains(&self)                      -> &Vec<Chain> { &self.chains }
	pub fn chains_as_mut(&mut self)           -> &mut Vec<Chain> { &mut self.chains }
	pub fn len(&self)                         -> usize { self.chains.len() }
}

impl Index<usize> for PDBModel {
	type Output = Chain;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.chains[idx]
	}
}

//impl Index<&str> for PDBModel {
//	type Output = Vec<Chain>;
//
//	fn index(&self, chainid: &str) -> &Self::Output {
//		self.chains.iter().filter(|chain| chain.chain_id_as_str() == chainid).collect()
//	}
//}
