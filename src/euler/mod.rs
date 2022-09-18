pub fn run() {
    euler_1();
    euler_2();
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

fn euler_2() -> i32 {
    const LIMIT: i32 = 4_000_000;
    let mut fibonacci = vec![];
    fibonacci.push(1);
    fibonacci.push(2);
    let mut i = 2;
    let mut total = 2;
    while fibonacci[fibonacci.len() - 1] < LIMIT {
        let value = fibonacci[i - 1] + fibonacci[i - 2];
        if value % 2 == 0 {
            total += &value;
        }
        fibonacci.push(value);
        i += 1;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::euler_1;
    use super::euler_2;

    #[test]
    fn test_euler_1() {
        let result = euler_1();
        assert_eq!(result, 233168);
    }
    #[test]
    fn test_euler_2() {
        let result = euler_2();
        assert_eq!(result, 4613732);
    }
}
