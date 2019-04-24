pub fn nth(n: u32) -> u32 {
    
    /*let a:[u32; 25] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let dipilih = a[n as usize];
    return dipilih;*/

	let mut vec: Vec<_> = (2..200000).collect();

    for p in 2..200000 {
        vec.retain(|&x| x <= p || x % p != 0);
    }

    return vec[n as usize];		
}
		


