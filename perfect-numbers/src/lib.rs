#[derive(Debug, PartialEq, Eq)]



pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let mut jumlah = 0;

    for i in 1..num {
      if num % i == 0 {
      	jumlah += i
      } else{
      	jumlah = jumlah
      }  
    }

    if num == jumlah {
    	Some(Classification::Perfect)
    }else if num > jumlah {
    	Some(Classification::Deficient)
    }else{
    	Some(Classification::Abundant)
    }

    
}
