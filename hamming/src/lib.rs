/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
  	let mut count = 0;

    if s1 == s2 {
    	Some(0)
    }else if s1.len() != s2.len() {
    	None
    }else{
    	let vec_s1 : Vec <char> = s1.chars().collect();
    	let vec_s2 : Vec <char> = s2.chars().collect();
    	
    	for i in 0..vec_s1.len(){
    		if vec_s1[i] != vec_s2[i]{
    			count += 1;
    		}else{
    			count = count
    		}
    	}

    	Some(count)
    }
}
