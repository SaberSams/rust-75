pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut zero_count: usize = 0;
    let mut index: usize = 0;

    // count zeros and shift everything back by the amount of zeros found
    loop {
        match nums[index]{
            0 => zero_count += 1,
            _ => {
                nums[index - zero_count] = nums[index];
            }
        }
        index += 1;
        if index >= nums.len() { break }
    }
    index -= zero_count;
    // add the zeros back at the end
    while index < nums.len() {
        nums[index] = 0;
        index += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let mut v = vec![0,1,0,3,12];
        move_zeroes(&mut v);
        assert_eq!(v, vec![1,3,12,0,0]);
    }

    #[test]
    fn example_2() {
        let mut v = vec![0];
        move_zeroes(&mut v);
        assert_eq!(v, vec![0]);
    }

    // #[test]
    // fn example_3() {
    //     let result = reverse_vowels(String::from(" "));
    //     assert_eq!(result, String::from(" "));
    // }

    // #[test]
    // fn example_4() {
    //     let result = reverse_vowels(String::from("aA"));
    //     assert_eq!(result, String::from("Aa"));
    // }
    
    // #[test]
    // fn example_5() {
    //     let result = reverse_vowels(String::from(".,"));
    //     assert_eq!(result, String::from(".,"));
    // }

}

