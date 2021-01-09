pub struct Mask<'a>(pub &'a str);

impl<'a> Mask<'a> {
    pub fn apply(self: &Mask<'a>, n: u64) -> u64 {
        n & gen_and_mask(self.0) | gen_or_mask(self.0)
    }
}

fn gen_and_mask(s: &str) -> u64 {
    let s = str::replace(s, "X", "1");
    u64::from_str_radix(&s.to_string(), 2).unwrap()
}

fn gen_or_mask(s: &str) -> u64 {
    let s = str::replace(s, "X", "0");
    u64::from_str_radix(&s.to_string(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static MASK: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";

    #[test]
    fn test_apply_mask() {
        assert_eq!(Mask(MASK).apply(11), 73);
        assert_eq!(Mask(MASK).apply(101), 101);
        assert_eq!(Mask(MASK).apply(0), 64);
    }

    #[test]
    fn test_gen_and_mask() {
        assert_eq!(
            gen_and_mask(MASK),
            0b111111111111111111111111111111111101_u64
        )
    }

    #[test]
    fn test_gen_or_mask() {
        assert_eq!(
            gen_or_mask(MASK),
            0b000000000000000000000000000001000000_u64
        )
    }
}
