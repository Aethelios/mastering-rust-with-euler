
fn main() {
    let mut primes = vec![2];
    let mut i = 3;
    let max = 150_000;

    while i < max {
    if primes.iter().all(|x| i % x != 0) { 
        primes.push(i);
    }
    i += 2;
    }

  let index = 10_000;
  let result = primes[index];

  println!("The {}th prime is {}", index + 1, result);
}