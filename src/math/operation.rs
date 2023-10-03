pub fn distance3d(lhs: &[f32; 3], rhs: &[f32; 3]) -> f32 {
	let mut retval = 0_f32;
	for idim in 0..3 {
		retval += (lhs[idim] - rhs[idim]).powi(2);
	}
	retval.sqrt()
}
