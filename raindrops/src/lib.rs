// Dibantu oleh Rama Lesmana


pub fn raindrops(n: u32) -> String {
    let mut output = "".to_string();
    let tri = "Pling".to_string();
    let five = "Plang".to_string();
    let seven = "Plong".to_string();


    if n % 3 == 0{
    	output.push_str(&tri); 
    }

    if n % 5 == 0{
    	output.push_str(&five);
	}

	if n % 7 == 0{
		output.push_str(&seven);
	}

	if n % 3 != 0 {
		if n%5 != 0 {
			if n%7 != 0{
				return n.to_string();
			}
		}
	}

	return output;
}
