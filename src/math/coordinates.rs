use num::Float;

pub struct Coordinates<T: Float> {
	xs: Vec<T>,
	ys: Vec<T>,
	zs: Vec<T>,
}

impl<T: Float> Coordinates<T> {
	pub fn new(xs: Vec<T>, ys: Vec<T>, zs: Vec<T>) -> Self {
		Self {
			xs,
			ys,
			zs,
		}
	}

	pub fn atom(&self, index: usize) -> Option<[T; 3]> {
		let x = match self.xs.get(index) {Some(val) => val.clone(), None => return None,};
		let y = match self.ys.get(index) {Some(val) => val.clone(), None => return None,};
		let z = match self.zs.get(index) {Some(val) => val.clone(), None => return None,};
		Some([x, y, z])
	}
}

