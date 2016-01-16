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
    fn test_35() {
        assert_eq!(is_prime(35), false);
    }

    #[test]
    fn test_first_30() {
        let result: Vec<i64> = (1..30).filter(|&x| is_prime(x) == true).collect();
        let expected: Vec<i64> = vec![2,3,5,7,11,13,17,19,23,29];
        assert_eq!(result, expected);
    }
}
