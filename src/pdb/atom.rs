use arrayvec::{ArrayVec, ArrayString};



pub struct Atom {
	serial    : i32,
	name      : ArrayString< 4 >,
	altloc    : ArrayString< 1 >,
	icode     : ArrayString< 1 >,
	coordinate: ArrayVec<f64, 3>,
	occupancy : f64,
	tempfactor: f64,
	element   : ArrayString< 2 >,
	charge    : ArrayString< 2 >,
}


impl Atom {

	pub fn new(
		serial    : i32,
		name      : ArrayString< 4 >,
		altloc    : ArrayString< 1 >,
		icode     : ArrayString< 1 >,
		coordinate: ArrayVec<f64, 3>,
		occupancy : f64,
		tempfactor: f64,
		element   : ArrayString< 2 >,
		charge    : ArrayString< 2 >) -> Self {
		Self {
			serial,
			name,
			altloc,
			icode,
			coordinate,
			occupancy,
			tempfactor,
			element,
			charge,
		}
	}

	pub fn record_name(&self)           -> &str {"ATOM"}
	pub fn atom_number(&self)           -> i32  {self.serial}
	pub fn atom_name(&self)             -> ArrayString< 4 > {self.name}
	pub fn atom_name_as_str(&self)      -> &str {&self.name}
	pub fn altloc(&self)                -> ArrayString< 1 > {self.altloc}
	pub fn altloc_as_str(&self)         -> &str {&self.altloc}
	pub fn inscode(&self)               -> ArrayString< 1 > {self.icode}
	pub fn inscode_as_str(&self)        -> &str {&self.icode}
	pub fn coordinate(&self)            -> &ArrayVec<f64, 3> {&self.coordinate}
	pub fn occupancy(&self)             -> f64  {self.occupancy}
	pub fn tempfactor(&self)            -> f64  {self.tempfactor}
	pub fn element_symbol(&self)        -> ArrayString< 2 > {self.element}
	pub fn element_symbol_as_str(&self) -> &str {&self.element}
	pub fn charge(&self)                -> ArrayString< 2 > {self.charge}
	pub fn charge_as_str(&self)         -> &str {&self.charge}
}

