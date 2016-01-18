use std::collections::HashMap;

pub fn project_euler_14(my_num: i64) -> i64 {
    // using i64 because i32 overflows 

    // The solution is actually faster without
    // the HashMap!
    let mut my_map: HashMap<i64, i64> = HashMap::new();

    let mut my_max = 0;
    let mut max_val = 0;

    for i in 1..my_num {
        let mut n = i;
        let mut count = 1;
        while n != 1 {
            if my_map.contains_key(&n) {
                count = count + my_map.get(&n).unwrap();
                break;
            }

            count += 1;
            if n % 2 == 0 {
                n = n / 2;
            } else {
                n = n * 3 + 1;
            }
        }
        my_map.insert(i, count);

        if count > my_max {
            my_max = count;
            max_val = i;
        }
    }

    return max_val;
}
