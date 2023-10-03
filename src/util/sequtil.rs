
fn convert_aa_3_to_1(aa_3letter: &str) -> Result<char, String> {
	match aa_3letter {
		"ALA" => Ok('A'),
		"CYS" => Ok('C'),
		"ASP" => Ok('D'),
		"GLU" => Ok('E'),
		"PHE" => Ok('F'),
		"GLY" => Ok('G'),
		"HIS" => Ok('H'),
		"ILE" => Ok('I'),
		"LYS" => Ok('K'),
		"LEU" => Ok('L'),
		"MET" => Ok('M'),
		"ASN" => Ok('N'),
		"PRO" => Ok('P'),
		"GLN" => Ok('Q'),
		"ARG" => Ok('R'),
		"SER" => Ok('S'),
		"THR" => Ok('T'),
		"VAL" => Ok('V'),
		"TRP" => Ok('W'),
		"TYR" => Ok('Y'),
		// modified residue
		"NLE" => Ok('.'),
		"M3L" => Ok('.'),
		// DNA residue
		"DC"  => Ok('.'),
		"DA"  => Ok('.'),
		"DG"  => Ok('.'),
		"DT"  => Ok('.'),
		"C"   => Ok('.'),
		"A"   => Ok('.'),
		"G"   => Ok('.'),
		"T"   => Ok('.'),
		_     => Err(format!("Unknown residue name, {} exists.", aa_3letter).to_string()),
	}
}

pub fn aa_3_to_1(seq_3letter: &Vec<String>) -> Result<Vec<char>, String> {
	return seq_3letter.iter().map(|aa_3letter| convert_aa_3_to_1(&aa_3letter)).collect()
}
