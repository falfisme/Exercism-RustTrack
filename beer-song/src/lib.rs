

// Dibantu oleh Rama Lesmana

pub fn verse(n: i32) -> String {
	let mut bottle = "bottle".to_string();
    let mut nextbottle = "bottle".to_string();
    let mut one = "one".to_string();
    let mut nextcount = format!("{}",n-1);
    if n != 1{
    	bottle.push_str("s");
    }
 
    if n - 1 != 1{
    	nextbottle.push_str("s");
    }
    if n == 0 {
    	let zero = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
    	return zero.to_string();
    }

    if n == 1{
    	one = "it".to_string();
    	nextcount = "no more".to_string();
    }

    return format!("{} {} of beer on the wall, {} {} of beer.\nTake {} down and pass it around, {} {} of beer on the wall.\n", n, bottle, n, bottle, one , nextcount, nextbottle);
}

pub fn sing(start: i32, end: i32) -> String {
   let mut song = "".to_string();
   let mut i = start;
   loop {
  		if i == end {
  			song = format!("{}{}",song,&verse(i));
  			break;
  		} else {
  			song = format!("{}{}\n",song,&verse(i));
  			i = i - 1;
  			continue;
  		}
   }
   return song;
}
