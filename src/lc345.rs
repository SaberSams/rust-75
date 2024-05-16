pub fn reverse_vowels(s: String) -> String {
    if s.len() == 1 {
        return s
    }
    let mut chars: Vec<char> = s.chars().collect();
    // create a pointer to the first and last element
    let mut first: usize = 0;
    let mut last: usize = chars.len() - 1;
    println!("first {}, last {}", first, last);
    // while the first pointer is less then the last pointer keep swaping
    loop {
        // find the first vowel
        loop {
            match chars.get(first){
                Some('a') |
                Some('e') | 
                Some('i') | 
                Some('o') | 
                Some('u') |
                Some('A') |
                Some('E') |
                Some('I') |
                Some('O') |
                Some('U') |
                None => break,
                _ => first += 1,
            }
        }
        loop {
            println!("last {}", last);
            match chars.get(last){
                Some('a') |
                Some('e') | 
                Some('i') | 
                Some('o') | 
                Some('u') |
                Some('A') |
                Some('E') |
                Some('I') |
                Some('O') |
                Some('U') |
                None => break,
                _ => {
                    if last == 0 { break }
                    last -= 1
                },
            }

        }
        println!("first {}, last {}", first, last);
        if first >= last { break };
        let temp:char = chars[last];
        chars[last] = chars[first];
        chars[first] = temp;
        first += 1;
        last -= 1;
    }
    chars.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let result = reverse_vowels(String::from("hello"));
        assert_eq!(result, String::from("holle"));
    }

    #[test]
    fn example_2() {
        let result = reverse_vowels(String::from("leetcode"));
        assert_eq!(result, String::from("leotcede"));
    }

    #[test]
    fn example_3() {
        let result = reverse_vowels(String::from(" "));
        assert_eq!(result, String::from(" "));
    }

    #[test]
    fn example_4() {
        let result = reverse_vowels(String::from("aA"));
        assert_eq!(result, String::from("Aa"));
    }
    
    #[test]
    fn example_5() {
        let result = reverse_vowels(String::from(".,"));
        assert_eq!(result, String::from(".,"));
    }

}