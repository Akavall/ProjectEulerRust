use utilities::prime_utilities::is_prime;

pub fn project_euler_3(my_num: i64) -> i64 {
    let limit = (my_num as f64).sqrt() as i64 + 1;
    
    let mut largest: i64 = 0;
    for i in 0..limit {
        if is_prime(i) && my_num % i == 0 {
            largest = i;
        }
    }
    largest
}
