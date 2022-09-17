pub fn run() {
    euler_1();
}
fn euler_1() -> i32 {
    let mut i = 0;
    let mut total = 0;
    while i < 1000 {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
            i += 1;
            continue;
        }
        i += 1;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::euler_1;

    #[test]
    fn test_euler_1() {
        let result = euler_1();
        assert_eq!(result, 233168);
    }
}
