use arrayvec::ArrayString;

pub struct Atom {
	serial    : i32,
	name      : ArrayString<4>,
	coordinate: [f32; 3],
	charge    : f32,
	radius    : f32,
}


impl Atom {

	pub fn new(
		serial    : &str,
		name      : &str,
		coordinate: [&str; 3],
		charge    : &str,
		radius    : &str) -> Result<Self, String> {

		Ok(Self {
			serial: serial.parse::<i32>().map_err(|err| err.to_string())?,
			name: ArrayString::<4>::from(name).map_err(|err| err.to_string())?,
			coordinate: [
				coordinate[0].parse::<f32>().map_err(|err| err.to_string())?,
				coordinate[1].parse::<f32>().map_err(|err| err.to_string())?,
				coordinate[2].parse::<f32>().map_err(|err| err.to_string())?
			],
			charge: charge.parse::<f32>().map_err(|err| err.to_string())?,
			radius: radius.parse::<f32>().map_err(|err| err.to_string())?,
		})
	}

	pub fn record_name(&self)           -> &str {"ATOM"}
	pub fn atom_number(&self)           -> i32  {self.serial}
	pub fn atom_name(&self)             -> &str {&self.name}
	pub fn coordinate(&self)            -> &[f32; 3] {&self.coordinate}
	pub fn charge(&self)                -> f32 {self.charge}
	pub fn radius(&self)                -> f32 {self.radius}
}

