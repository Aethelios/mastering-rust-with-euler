use mylib::{self, is_palindrome};
fn main(){
    let mut biggest_palindrome = 0;
    for p1 in 0..999 {
        for p2 in 0..999 {
            if is_palindrome(p1*p2) && (p1*p2)>biggest_palindrome{
                biggest_palindrome = p1*p2;
            }
        }
    }
    print!("{biggest_palindrome}")
}