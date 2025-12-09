#[cfg(test)]
mod tests {
    use crate::is_invalid_id_part1;

    #[test]
    fn test_range_11_22() {
        let range = 11..=22;
        let invalid: Vec<u64> = range.clone().filter(|&n| is_invalid_id_part1(n)).collect();
        assert_eq!(invalid, vec![11, 22]);
    }

    #[test]
    fn test_range_95_115() {
        let range = 95..=115;
        let invalid: Vec<u64> = range.clone().filter(|&n| is_invalid_id_part1(n)).collect();
        assert_eq!(invalid, vec![99]);
    }

    #[test]
    fn test_range_998_1012() {
        let range = 998..=1012;
        let invalid: Vec<u64> = range.clone().filter(|&n| is_invalid_id_part1(n)).collect();
        assert_eq!(invalid, vec![1010]);
    }

    #[test]
    fn test_range_1188511880_1188511890() {
        let range = 1188511880..=1188511890;
        let invalid: Vec<u64> = range.clone().filter(|&n| is_invalid_id_part1(n)).collect();
        assert_eq!(invalid, vec![1188511885]);
    }

    #[test]
    fn test_range_222220_222224() {
        let range = 222220..=222224;
        let invalid: Vec<u64> = range.clone().filter(|&n| is_invalid_id_part1(n)).collect();
        assert_eq!(invalid, vec![222222]);
    }

    #[test]
    fn test_range_1698522_1698528() {
        let range = 1698522..=1698528;
        let invalid: Vec<u64> = range.clone().filter(|&n| is_invalid_id_part1(n)).collect();
        assert_eq!(invalid, Vec::<u64>::new());
    }

    #[test]
    fn test_range_446443_446449() {
        let range = 446443..=446449;
        let invalid: Vec<u64> = range.clone().filter(|&n| is_invalid_id_part1(n)).collect();
        assert_eq!(invalid, vec![446446]);
    }

    #[test]
    fn test_range_38593856_38593862() {
        let range = 38593856..=38593862;
        let invalid: Vec<u64> = range.clone().filter(|&n| is_invalid_id_part1(n)).collect();
        assert_eq!(invalid, vec![38593859]);
    }
}
