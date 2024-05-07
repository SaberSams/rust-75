// 1431. Kids With the Greatest Number of Candies
// There are n kids with candies. You are given an integer array candies,
// where each candies[i] represents the number of candies the ith kid has,
// and an integer extraCandies, denoting the number of extra candies that you have.
//
// Return a boolean array result of length n, where result[i] is true if,
// after giving the ith kid all the extraCandies, they will have the greatest
// number of candies among all the kids, or false otherwise.
// Note that multiple kids can have the greatest number of candies.

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
mod tests {
    use super::*;

    #[test]
    fn kids_with_candies_test() {
        let result = kids_with_candies(vec![2, 3, 5, 1, 3], 3);
        assert_eq!(result, vec![true, true, true, false, true]);
    }
}
