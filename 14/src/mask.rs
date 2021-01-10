#[derive(Debug)]
pub struct Mask<'a>(pub &'a str);

impl<'a> Mask<'a> {
    pub fn apply(self: &Mask<'a>, n: u64) -> u64 {
        n & gen_and_mask(self.0) | gen_or_mask(self.0)
    }

    pub fn apply2(self: &Mask<'a>, n: u64) -> Vec<usize> {
        let result: String = format!("{:036b}", n)
            .chars()
            .zip(self.0.chars())
            .map(|(n, m)| match m {
                '0' => n,
                '1' => '1',
                _ => m,
            })
            .collect();
        let floating_indices: Vec<usize> = self
            .0
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == 'X' { Some(i) } else { None })
            .collect();
        gen_addresses(&result, &floating_indices)
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

fn gen_addresses(s: &str, floating_indices: &[usize]) -> Vec<usize> {
    let index = floating_indices[0];
    let mut s1 = s.chars().collect::<Vec<_>>();
    let mut s2 = s.chars().collect::<Vec<_>>();
    s1[index] = '0';
    s2[index] = '1';
    if floating_indices.len() == 1 {
        let s1: String = s1.iter().collect();
        let s2: String = s2.iter().collect();
        vec![
            usize::from_str_radix(&s1, 2).unwrap(),
            usize::from_str_radix(&s2, 2).unwrap(),
        ]
    } else {
        let s1: String = s1.iter().collect();
        let s2: String = s2.iter().collect();
        let mut s1_variants = gen_addresses(&s1, &floating_indices[1..]);
        let s2_variants = gen_addresses(&s2, &floating_indices[1..]);
        s1_variants.extend(&s2_variants);
        s1_variants
    }
}

#[cfg(test)]
mod tests_part1 {
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

#[cfg(test)]
mod tests_part2 {
    use super::*;

    static MASK1: &str = "000000000000000000000000000000X1001X";
    static MASK2: &str = "00000000000000000000000000000000X0XX";

    #[test]
    fn test_apply2() {
        assert_eq!(Mask(MASK1).apply2(42), vec![26, 27, 58, 59]);
        assert_eq!(Mask(MASK2).apply2(26), vec![16, 17, 18, 19, 24, 25, 26, 27]);
    }
}
