pub fn check(candidate: &str) -> bool {
    let tulisan = candidate.to_lowercase();
    let char_vec: Vec<char> = tulisan.chars().collect();
    let mut count = 0;

    for i in 0..char_vec.len(){
    	let tes = char_vec[i];
    	let titik = i;
    	for j in 0..char_vec.len()-1{
    		if char_vec[j] == tes{
    			if titik == j {
    				count = count;
    			}else{
    				if char_vec[j] == ' ' || char_vec[j] == '-'{
    					count = count;
    				}else {
    					count += 1;
    				}
    			}
    		}
    		else{
    			count = count;
    		}
    	}
    	
    }

    let mut bolean = true;

    if count > 0 {
    	bolean = false
    }else{
    	bolean = true;
    	count = 0;
    }



    	return bolean;
}
