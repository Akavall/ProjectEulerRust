pub mod prime_utilities {
    pub fn is_prime(x: i64) -> bool {
        if x == 0 || x == 1 { return false; }
        if x == 2 { return true; }
        if x % 2 == 0 { 
            return false;
        } else {
            let limit = (x as f64).sqrt() as i64 + 2;
            let mut i = 3;
            while i < limit {
                if x % i == 0 { return false; }
                i += 2;
            }
        }

        return true;
    }

    #[test]
    fn test_35_prime() {
        assert_eq!(is_prime(35), false);
    }

    #[test]
    fn test_first_30_prime() {
        let result: Vec<i64> = (1..30).filter(|&x| is_prime(x) == true).collect();
        let expected: Vec<i64> = vec![2,3,5,7,11,13,17,19,23,29];
        assert_eq!(result, expected);
    }
}

pub mod palindrome_utilities {
    pub fn is_palindrome(mut a: i64) -> bool {
        let mut my_vec = vec![];
        while a > 0{
            my_vec.push(a % 10);
            a = a / 10;
        }
        let mut rev_vec = my_vec.to_owned();
        rev_vec.reverse();
        let my_bool = rev_vec == my_vec;
        return my_bool;
    }

    #[test]
    fn test_0_is_palindrome() {
        assert_eq!(is_palindrome(0), true);
    }

    #[test]
    fn test_123_is_palindrome() {
        assert_eq!(is_palindrome(123), false);
    }

    #[test]
    fn test_121_is_palindrome() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn test_906609_is_palindrome() {
        assert_eq!(is_palindrome(906609), true);
    }

}
