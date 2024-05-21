pub fn reverse_words(s: String) -> String {
    s.split(' ')
        .rev()
        .map(|s| s.trim())
        .filter(|&s| s != "")
        .fold(String::from(""), |i, j| i + j + " ")
        .trim()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = String::from("the sky is blue");
        let output = String::from("blue is sky the");
        let result = reverse_words(input);
        assert_eq!(output, result);
    }

    #[test]
    fn example_2() {
        let input = String::from("  hello world  ");
        let output = String::from("world hello");
        let result = reverse_words(input);
        assert_eq!(output, result);
    }

    #[test]
    fn example_3() {
        let input = String::from("a good   example");
        let output = String::from("example good a");
        let result = reverse_words(input);
        assert_eq!(output, result);
    }
}
