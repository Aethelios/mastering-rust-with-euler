use std::str::Chars;


pub fn fibonacci (n: i32) -> i32 {
    if n <= 0 {
          return 0;
    } else if n== 1{
          return 1;
} else {
    return fibonacci (n-1)  + fibonacci(n-2);
 }
}

pub fn largest_prime_factor(mut n: u64) -> u64 {

    let mut largest = 0;
    let mut divisor = 2;
  
    while n > 1 {
      if n % divisor == 0 {
        largest = divisor;
        n /= divisor;
      } else {
        divisor += 1; 
      }
    }
  
    largest
  }

pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;
  
    while n > 1 {
      while n % divisor == 0 {
        factors.push(divisor);
        n /= divisor;
      }
      divisor += 1;
    }
  
    factors
}

pub fn is_palindrome(num: i32) -> bool {
    let num_str = num.to_string();
    let chars: Chars = num_str.chars();
    for (i, j) in chars.clone().rev().zip(chars) {
      if i != j {
        return false;
      }
    }
  
    true
  }

pub fn smallest_mult(max: u64) -> u64 {

    let mut result = 1;
    
    for i in 2..=max {
      result = lcm(result, i);
    }
  
    result
  }
  
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
      a
    } else {
      gcd(b, a % b)
    }
  }
  
pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
  }



  

pub fn li(n: u64) -> f64 {
    let mut sum = 0.0;
    for i in 2..n {
        sum += 1.0 / (i as f64).ln();
    }
    sum
  }
  
pub fn prime_li(n: u64) -> u64 {
    li(n as u64).round() as u64
  }
  
pub fn prime_poly(n: u64) -> u64 {
    (n as f64 * (n as f64).ln()) as u64 + (n as f64 * (n as f64).ln().ln()) as u64 
  }
  
pub fn prime_analytic(n: u64) -> u64 {
    let n = n as f64;
    let log_n = n.ln();
    let log_log_n = log_n.ln();
    
    ((n * log_n) + (n * log_log_n) - n).floor() as u64
  }
  
