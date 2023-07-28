use num_bigint::BigUint; 

fn main() {
  let two: BigUint = BigUint::from(2u32);
  let result = two.pow(1000);

  let sum:BigUint = result.to_str_radix(10)
                 .chars()
                 .map(|c| c.to_digit(10).unwrap() as u32)
                 .sum();

  println!("{}", sum);
}