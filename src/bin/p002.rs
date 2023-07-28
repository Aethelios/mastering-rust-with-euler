use mylib;
fn main() {
    let mut n = 2;
    let mut sum = 0;
    while mylib::fibonacci(n)<4000000 {
        if mylib::fibonacci(n) % 2 ==0  {
            sum += mylib::fibonacci(n);
        }
        n+= 1;
    }
    println!("{sum}");
}
// i know about the formula but fuck it.