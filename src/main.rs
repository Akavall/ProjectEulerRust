mod utilities;
mod project_euler_3;
mod project_euler_4;
mod project_euler_7;
mod project_euler_10;
mod project_euler_14;
mod n_primes_under;

use project_euler_3::project_euler_3;
use project_euler_4::project_euler_4;
use project_euler_7::project_euler_7;
use project_euler_10::project_euler_10;
use project_euler_14::project_euler_14;
use n_primes_under::n_primes_under;

fn main() {
    println!("Project Euler 3: {}", project_euler_3(600851475143));
    println!("Project Euler 4: {}", project_euler_4(4));
    println!("Project Euler 7: {}", project_euler_7(10001));
    println!("Project Euler 10: {}", project_euler_10(2000000));
    println!("Project Euler 14: {}", project_euler_14(1000000));
    println!("n primes under: {}", n_primes_under(16_000_000));

}
