use utilities::prime_utilities::is_prime;

pub fn project_euler_3(my_num: i64) -> i64 {
    let limit = (my_num as f64).sqrt() as i64 + 1;
    
    let largest = (0..limit).filter(|&x| is_prime(x) && my_num % x == 0).max().unwrap();
    largest
}
