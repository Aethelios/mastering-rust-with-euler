fn main() {
    let sum = 1000;
  
    for a in 1..sum {
      for b in a..sum { 
        let c = sum - a - b;
        if c*c == a*a + b*b {
          println!("a: {} b: {} c: {}", a, b, c);
          println!("product: {}", a * b * c);
        }
      }
    }
  }