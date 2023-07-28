use num::pow;

fn get_next(num: u32) -> u32 {
  let mut sum = 0;
  let mut n = num;
  
  while n > 0 {
    let digit = n % 10;
    sum += pow(digit, 2);
    n /= 10;
  }

  sum
}
fn main(){
let mut count = 0;
for i in 1..10_000_000 {
  let mut current = i;
  
  while current != 1 && current != 89 {
    current = get_next(current);
  }
  
  if current == 89 {
    count += 1;
  }
}

println!("{}", count);
}