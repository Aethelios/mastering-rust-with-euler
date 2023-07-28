fn main(){
    let mut n_sqr_sum:i128  = 0;
    let mut n_sum_sqr:i128  = 0;
    for i in 1..101 {
        n_sqr_sum += i*i;
        n_sum_sqr += i;
    }
    n_sum_sqr *= n_sum_sqr;
    println!("{}",n_sum_sqr-n_sqr_sum)

}