use utilities::prime_utilities::is_prime;

pub fn project_euler_10(my_num: i64) -> i64 {
    
    let my_sum = (0..my_num).filter(|&x| is_prime(x)).fold(0, |acc, x| acc + x );
    my_sum

}
