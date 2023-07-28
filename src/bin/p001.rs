fn main(){

  let mut n = 1;
  let mut sum = 0;
      
  while n < 1000 {
    if n % 5 == 0 || n % 3 == 0 {
      sum += n;
    }
    n += 1;
  }
  println!("{sum}");
  
}
