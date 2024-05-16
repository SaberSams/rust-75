
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;
    flowerbed.insert(0, 0);
    flowerbed.push(0);

    let mut count = 0;
    for i in 1..flowerbed.len() - 1 {
        if flowerbed[i - 1] | flowerbed[i] | flowerbed[i + 1] == 0 {
            flowerbed[i] = 1;
            count += 1;
        }
    }
    count >= n
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let result = can_place_flowers(vec![1,0,0,0,1], 1);
        assert_eq!(result, true);
    }

    #[test]
    fn example_2() {
        let result = can_place_flowers(vec![1,0,0,0,1], 2);
        assert_eq!(result, false);
    }

}
