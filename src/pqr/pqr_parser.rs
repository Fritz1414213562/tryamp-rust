use std::io::{Read, BufRead, BufReader};
use super::pqrmodel::PQRModel;
//use super::chain::Chain;
use super::residue::Residue;
use super::atom::Atom;



pub struct PQRParser;

impl PQRParser {

	pub fn new() -> Self {
		Self
	}
	
	pub fn read_model<R: Read>(&mut self, file: R) -> Result<PQRModel, String> {
		let reader = &mut BufReader::<R>::new(file);
		let mut model : PQRModel = PQRModel::new();

		let mut curr_resid: i32 = 0;
		let mut curr_residue: Residue = Residue::new();
		let mut is_at_first = true;

		for result in reader.lines() {
			let line = match result {
				Ok(buffer) => buffer,
				Err(err) => return Err(err.to_string()),
			};

			if line.is_empty() { continue; }

			if util::starts_with("ATOM", &line) {

				let words: Vec<&str> = line.split_whitespace().collect();

				let atom_resid: i32 = words[4].parse::<i32>().map_err(|err| err.to_string())?;
				if is_at_first {
					curr_resid = atom_resid;
					curr_residue = util::build_residue(words[3], curr_resid)?;
					is_at_first = false;
				}
				if curr_resid != atom_resid {
					model.residues_as_mut().push(curr_residue);
					curr_resid = atom_resid;
					curr_residue = util::build_residue(words[3], curr_resid)?;
				}

				let curr_atom: Atom = util::build_atom(words)?;
				curr_residue.atoms_as_mut().push(curr_atom);
			}
		}
		if !curr_residue.atoms().is_empty() { model.residues_as_mut().push(curr_residue); }
		Ok(model)
	}

}



mod util {
	use super::super::residue::Residue;
	use super::super::atom::Atom;

	pub fn starts_with(kind: &str, line: &str) -> bool {
		if line.len() < kind.len() { false }
		else { &line[..kind.len()] == kind }
	}

	pub fn build_atom(words: Vec<&str>) -> Result<Atom, String> {
		if words.len() < 10 {
			return Err(format!("ATOM line does not have enough length. \n{}", words.join(" ")))
		}
		if words[0] != "ATOM" {
			return Err(format!("This line is not 'ATOM' line. \n{}", words.join(" ")))
		}

		Atom::new(words[1], words[2], [words[5], words[6], words[7]], words[8], words[9])
	}

	pub fn build_residue(resin: &str, atom_resid: i32) -> Result<Residue, String> {
		let mut retval: Residue = Residue::new();
		retval.change_residue_name(resin)?;
		retval.change_residue_id(atom_resid);
		Ok(retval)
	}

}
