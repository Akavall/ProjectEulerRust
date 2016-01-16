use utilities::prime_utilities::is_prime;

pub fn project_euler_7(stop_at_n: i64) -> i64 {
    
    let mut n: i64 = 0;
    let mut prime_count: i64 = 0;
    while prime_count < stop_at_n {
        n += 1;
        if is_prime(n) { prime_count += 1; }
    }
    return n;
}
