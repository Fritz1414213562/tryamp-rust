use arrayvec::{ArrayVec, ArrayString};
use std::io::{Read, BufRead, BufReader};
use super::pdbmodel::PDBModel;
use super::chain::Chain;
use super::residue::Residue;
use super::atom::Atom;


//mod atom;
//mod residue;
//mod chain;
//mod pdbmodel;


pub struct PDBParser;

impl PDBParser {

	pub fn new() -> Self {
		Self
	}
	
	pub fn read_model<R: Read>(&mut self, file: R) -> Result<Vec<PDBModel>, String> {
		let reader = &mut BufReader::<R>::new(file);
		let mut retval: Vec<PDBModel> = Vec::<PDBModel>::new();
		let mut model : PDBModel = PDBModel::new(Vec::<Chain>::new());

		let mut curr_resid: i32 = 0;
		let mut curr_chainid: ArrayString< 1 > = ArrayString::< 1 >::new();
		let mut curr_residue: Residue = Residue::new(Vec::<Atom>::new(), ArrayString::< 3 >::new(), 0);
		let mut curr_chain  : Chain = Chain::new(Vec::<Residue>::new(), ArrayString::< 1 >::new());
		let mut is_on_top_of_chain = true;

		for result in reader.lines() {
			let line = match result {
				Ok(buffer) => buffer,
				Err(err) => return Err(err.to_string()),
			};

			if line.is_empty() { continue; }
			if util::starts_with("END", &line) {
				retval.push(model);
				model = PDBModel::new(Vec::<Chain>::new());
			}
			if util::starts_with("TER", &line) {
				curr_chain.residues_as_mut().push(curr_residue);
				curr_chain.chain_id().push_str(&curr_chainid);
				model.chains_as_mut().push(curr_chain);
				curr_residue = Residue::new(Vec::<Atom>::new(), ArrayString::< 3 >::new(), 0);
				curr_chain = Chain::new(Vec::<Residue>::new(), ArrayString::< 1 >::new());
				is_on_top_of_chain = true;
			}
			if util::starts_with("ATOM", &line) {
				let atom_strings: ArrayVec<String, 15>;
				match util::read_atom(&line) {
					Ok(val) => atom_strings = val,
					Err(err) => return Err(err.to_string()),
				}

				let mut atom_resid: i32 = 0;
				match atom_strings[6].parse::<i32>() {
					Ok(val) => atom_resid = val,
					Err(err) => return Err(err.to_string()),
				}
				if is_on_top_of_chain {
					curr_resid = atom_resid;
					match util::build_residue(&atom_strings, curr_resid) {
						Ok(val) => curr_residue = val,
						Err(err) => return Err(err),
					}
					match ArrayString::< 1 >::from(&atom_strings[5]) {
						Ok(val) => curr_chainid = val,
						Err(err) => return Err(err.to_string()),
					}
					is_on_top_of_chain = false;
				}
				if curr_resid != atom_resid {
					curr_chain.residues_as_mut().push(curr_residue);
					curr_resid = atom_resid;
					match util::build_residue(&atom_strings, curr_resid) {
						Ok(val) => curr_residue = val,
						Err(err) => return Err(err),
					}
				}
				let curr_atom: Atom;
				match util::build_atom(&atom_strings) {
					Ok(val) => curr_atom = val,
					Err(err) => return Err(err),
				}
				curr_residue.atoms_as_mut().push(curr_atom);
			}
		}
		if retval.is_empty() { retval.push(model); }
		Ok(retval)
	}

}



mod util {
	use arrayvec::{ArrayVec, ArrayString};
	use super::super::residue::Residue;
	use super::super::atom::Atom;

	pub fn starts_with(kind: &str, line: &str) -> bool {
		if line.len() < kind.len() { false }
		else { &line[..kind.len()] == kind }
	}

	pub fn read_atom(line: &str) -> Result<ArrayVec<String, 15>, String> {
		if line.len() < 54 {
			return Err(format!("ATOM line does not have enough length. \n{}", line));
		}
		if &line[..6] != "ATOM  " {
			return Err(format!("This line is not 'ATOM' line. \n{}", line));
		}
		let mut retval = ArrayVec::<String, 15>::new();
		retval.push(line[ 0.. 6].trim().to_string()); // Record name
		retval.push(line[ 6..11].trim().to_string()); // Atom serial number
		retval.push(line[12..16].trim().to_string()); // Atom name
		retval.push(line[16..17].trim().to_string()); // Alternative location ID
		retval.push(line[17..20].trim().to_string()); // Residue name
		retval.push(line[21..22].trim().to_string()); // Chain ID
		retval.push(line[22..26].trim().to_string()); // Residue sequence number
		retval.push(line[27..28].trim().to_string()); // Code for insertion of residue
		retval.push(line[30..38].trim().to_string()); // Orthogonal coordinates for X in Angstroms
		retval.push(line[38..46].trim().to_string()); // Orthogonal coordinates for Y in Angstroms
		retval.push(line[46..54].trim().to_string()); // Orthogonal coordinates for Z in Angstroms
		if line.len() < 60 {retval.push("0.0".to_string());}   else { retval.push(line[54..60].trim().to_string()); } // Occupancy
		if line.len() < 66 {retval.push("999.9".to_string());} else { retval.push(line[60..66].trim().to_string()); } // tempFactor
		if line.len() < 78 {retval.push("  ".to_string());}    else { retval.push(line[76..78].trim().to_string()); }
		if line.len() < 80 {retval.push("  ".to_string());}    else { retval.push(line[78..80].trim().to_string()); }

		Ok(retval)
	}

	pub fn build_atom(atom_strings: &ArrayVec<String, 15>) -> Result<Atom, String> {
		let serial_id: i32;
		match atom_strings[1].parse::<i32>() {
			Ok(val) => serial_id = val,
			Err(err) => return Err(err.to_string()),
		}
		let atom_name: ArrayString::< 4 >;
		match ArrayString::< 4 >::from(&atom_strings[2]) {
			Ok(val) => atom_name = val,
			Err(err) => return Err(err.to_string()),
		}
		let alt_loc: ArrayString::< 1 >;
		match ArrayString::< 1 >::from(&atom_strings[3]) {
			Ok(val) => alt_loc = val,
			Err(err) => return Err(err.to_string()),
		}
		let ins_code: ArrayString::< 1 >;
		match ArrayString::< 1 >::from(&atom_strings[7]) {
			Ok(val) => ins_code = val,
			Err(err) => return Err(err.to_string()),
		}
		let coord_x: f64;
		match atom_strings[8].parse::<f64>() {
			Ok(val) => coord_x = val,
			Err(err) => return Err(err.to_string()),
		}
		let coord_y: f64;
		match atom_strings[9].parse::<f64>() {
			Ok(val) => coord_y = val,
			Err(err) => return Err(err.to_string()),
		}
		let coord_z: f64;
		match atom_strings[10].parse::<f64>() {
			Ok(val) => coord_z = val,
			Err(err) => return Err(err.to_string()),
		}
		let occup: f64;
		match atom_strings[11].parse::<f64>() {
			Ok(val) => occup = val,
			Err(err) => return Err(err.to_string()),
		}
		let tempf: f64;
		match atom_strings[12].parse::<f64>() {
			Ok(val) => tempf = val,
			Err(err) => return Err(err.to_string()),
		}
		let elem: ArrayString::< 2 >;
		match ArrayString::< 2 >::from(&atom_strings[13]) {
			Ok(val) => elem = val,
			Err(err) => return Err(err.to_string()),
		}
		let chg: ArrayString::< 2 >;
		match ArrayString::< 2 >::from(&atom_strings[14]) {
			Ok(val) => chg = val,
			Err(err) => return Err(err.to_string()),
		}

		Ok(Atom::new(
			serial_id,
			atom_name,
			alt_loc,
			ins_code,
			ArrayVec::<f64, 3>::from([coord_x, coord_y, coord_z]),
			occup,
			tempf,
			elem,
			chg
		))
	}

	pub fn build_residue(atom_strings: &ArrayVec<String, 15>, atom_resid: i32) -> Result<Residue, String> {
		let resiname: ArrayString::< 3 >;
		match ArrayString::< 3 >::from(&atom_strings[4]) {
			Ok(val) => resiname = val,
			Err(err) => return Err(err.to_string()),
		}
		Ok(Residue::new(Vec::<Atom>::new(), resiname, atom_resid))
	}

}
