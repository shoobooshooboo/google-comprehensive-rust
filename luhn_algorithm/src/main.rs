pub fn luhn(cc_number: &str) -> bool {
    let mut numCount = 0;
    for c in cc_number.chars(){
        if c.is_numeric(){
            numCount += 1;
        }
        else if !c.is_whitespace(){
            return false;
        }
    }

    if numCount <= 2{
        return false;
    }

    let mut sum = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            continue;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_no_spaces(){
        assert!(luhn("4263982640269299"));
        assert!(luhn("4539319503436467"));
        assert!(luhn("79927398713"));
    }

    #[test]
    fn test_one_digit(){
        assert!(!luhn("0"));
    }

    #[test]
    fn test_two_digits(){
        assert!(!luhn("00"));
    }

    #[test]
    fn test_empty(){
        assert!(!luhn(""));
        assert!(!luhn(" "));
    }

    #[test]
    fn test_non_number(){
        assert!(!luhn("real card"));
        assert!(!luhn("hmm"));
    }
}