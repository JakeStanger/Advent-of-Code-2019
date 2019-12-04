fn main() {
    let min = 193651;
    let max = 649729;

    let valid: Vec<i32> = (min..max).filter(|num| {
        let mut prev = 0;

        let mut has_double = false;

        let mut streak = 0;
        for c in num.to_string().chars() {
            let digit = c.to_digit(10).unwrap();

            if digit < prev {
                return false
            }

            if digit == prev {
                streak += 1;
            } else {
                if streak == 1 {
                    has_double = true;
                }

                streak = 0;
            }

            prev = digit;
        }

        has_double || streak == 1
    }).collect();

    println!("{}", valid.len());
}