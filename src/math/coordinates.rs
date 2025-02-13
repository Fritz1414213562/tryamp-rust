use num::Float;

#[derive(Clone)]
pub struct Coordinates<T: Float> {
	xs: Vec<T>,
	ys: Vec<T>,
	zs: Vec<T>,
}

impl<T: Float> Coordinates<T> {
	pub fn new(xs: Vec<T>, ys: Vec<T>, zs: Vec<T>) -> Result<Self, String> {
		if xs.len() != ys.len() || xs.len() != zs.len() {
			return Err(format!("Error: The size of xs, ys, and zs is inconsistent. {} {} {}", xs.len(), ys.len(), zs.len()).to_string())
		}
		Ok(Self {
			xs,
			ys,
			zs,
		})
	}

	pub fn atom(&self, index: usize) -> Option<[T; 3]> {
		let x = match self.xs.get(index) {Some(val) => val.clone(), None => return None,};
		let y = match self.ys.get(index) {Some(val) => val.clone(), None => return None,};
		let z = match self.zs.get(index) {Some(val) => val.clone(), None => return None,};
		Some([x, y, z])
	}

	pub fn len(&self) -> usize { self.xs.len() }

	pub fn xs(&self) -> &Vec<T> { &self.xs }
	pub fn ys(&self) -> &Vec<T> { &self.ys }
	pub fn zs(&self) -> &Vec<T> { &self.zs }

}
