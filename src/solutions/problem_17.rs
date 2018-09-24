const ONES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const TEENS: [&str; 10] = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const TENS: [&str; 10] = ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

fn format(x: u64) -> String {
    if x < 10 {
        String::from(ONES[x as usize])
    } else if x >= 10 && x < 20 {
        String::from(TEENS[(x % 10) as usize])
    } else if x < 100 {
        let tens = x / 10;
        let ones = x % 10;
        if ones == 0 {
            String::from(TENS[tens as usize])
        } else {
            format!("{}-{}", TENS[tens as usize], ONES[ones as usize])
        }
    } else if x < 1000 {
        let hundreds = x / 100;
        let remainder = x % 100;
        if remainder == 0 {
            format!("{} hundred", ONES[hundreds as usize])
        } else {
            format!("{} hundred and {}", ONES[hundreds as usize], format(remainder))
        }
    } else {
        String::from("one thousand")
    }
}

pub fn solve(up_to: u64) -> u64 {
    (1..=up_to)
        .map(format)
        .map(|s| s.replace(" ", "").replace("-", ""))
        .map(|s| s.len() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(format(13), "thirteen");
        assert_eq!(format(42), "forty-two");
        assert_eq!(format(40), "forty");
        assert_eq!(format(531), "five hundred and thirty-one");
        assert_eq!(format(400), "four hundred");
        assert_eq!(format(1000), "one thousand");
    }

    #[test]
    fn test_moar() {
        assert_eq!(solve(5), 19);
    }
}