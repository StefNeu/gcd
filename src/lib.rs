pub fn gcd(mut first: i128, mut second: i128) -> Result<i128, &'static str> {
    if first < 0 || second < 0 {
        return Err("Error, None of the numbers can be negative.");
    }
    if first < second {
        let cash: i128 = first;
        first = second;
        second = cash;
    }
    if second == 0 {
        return Ok(first);
    }
    gcd(second, first % second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result: i128 = gcd(6, 18).unwrap();
        assert_eq!(result, 6);

        result = gcd(24, 4).unwrap();
        assert_eq!(result, 4);

        let result = gcd(-10, 10);
        assert!(result.is_err())
    }
}
