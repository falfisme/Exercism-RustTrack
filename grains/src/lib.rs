pub fn square(s: u32)->u64 {
	if s == 0{
		panic!("Square must be between 1 and 64");
	}else if s > 64{
		panic!("Square must be between 1 and 64");
	}else {
		let sbaru = s - 1;
		return 2_u64.pow(sbaru);	
	}
	
}

pub fn total() -> u64 {
    return 18446744073709551615
}
