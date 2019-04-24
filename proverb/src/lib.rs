// Dibantu oleh Rama Lesmana

pub fn build_proverb(list: &[&str]) -> String {
    let v_len = list.len();
    let mut output = "".to_string();
   
    for i in 0..v_len{
    	if i == v_len - 1 {
    		output = format!("{}And all for the want of a {}.", output, list[0]);
    	
    	} else {
    		output = format!("{}For want of a {} the {} was lost.\n", output,list[i], list[i+1]);
    	}
    }
    return output.to_string();
}
