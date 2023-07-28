use primal::Sieve; 

fn main() {
  let ans = compute();
  println!("{}", ans); 
}

fn compute() -> String {
  
  let sieve = Sieve::new(1999999);
  let primes = sieve.primes_from(2);
  let sum: usize = primes.sum();
  sum.to_string()
}