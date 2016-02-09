use utilities::palindrome_utilities::is_palindrome;

pub fn project_euler_4(my_num: u32) -> i64 {
    let mut largest = 0;
    for i in 0..(10i64.pow(my_num)) {
        for j in 0..(10i64.pow(my_num)) {
            if j > i { break };
            let temp_prod = i * j;
            if is_palindrome(temp_prod) == true && temp_prod > largest {
                largest = i * j;
            }
        }
    }
    largest
}
