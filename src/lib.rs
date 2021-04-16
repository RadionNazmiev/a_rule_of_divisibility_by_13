fn thirt(n: i64) -> i64 {
    let mut x = n;
    loop {
        let temp = x
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .rev()
            .enumerate()
            .map(|(count, digit)| match count as i64 % 6 {
                0 => digit * 1,
                1 => digit * 10,
                2 => digit * 9,
                3 => digit * 12,
                4 => digit * 3,
                _ => digit * 4,
            })
            .sum();
        if temp == x { break } else { x = temp };
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_thirt() {
        assert_eq!(thirt(8529), 79);
        assert_eq!(thirt(85299258), 31);
        assert_eq!(thirt(5634), 57);
    }
}