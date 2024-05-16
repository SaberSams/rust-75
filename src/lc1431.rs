
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let greatest = match candies.iter().max() {
        Some(&max) => max,
        None => 0,
    };

    candies
        .iter()
        .map(|candy| candy + extra_candies >= greatest)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let result = kids_with_candies(vec![2, 3, 5, 1, 3], 3);
        assert_eq!(result, vec![true, true, true, false, true]);
    }

    #[test]
    fn example_2() {
        let result = kids_with_candies(vec![4, 2, 1, 1, 2], 1);
        assert_eq!(result, vec![true, false, false, false, false]);
    }

    #[test]
    fn example_3() {
        let result = kids_with_candies(vec![12, 1, 12], 10);
        assert_eq!(result, vec![true, false, true]);
    }
}
