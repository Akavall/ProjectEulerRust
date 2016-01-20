mod utilities;
mod project_euler_3;
mod project_euler_7;
mod project_euler_14;

use project_euler_3::project_euler_3;
use project_euler_7::project_euler_7;
use project_euler_14::project_euler_14;

fn main() {
    println!("Project Euler 3: {}", project_euler_3(600851475143));
    println!("Project Euler 7: {}", project_euler_7(10001));
    println!("Project Euler 14: {}", project_euler_14(1000000));
}
