pub fn solve(input: &str) -> String {
    let sum = input.lines().map(from_snafu).sum::<i64>();
    to_snafu(sum)
}

fn from_snafu(str: &str) -> i64 {
    let mut number = 0;
    for c in str.chars() {
        number = number * 5 + from_snafu_digit(c);
    }
    number
}

fn from_snafu_digit(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!(),
    }
}

fn to_snafu(mut number: i64) -> String {
    let mut string = String::new();

    let mut carry = 0;
    while number != 0 {
        let mut digit = number % 5 + carry;
        carry = 0;

        if digit > 2 {
            digit -= 5;
            carry = 1;
        }

        string += match digit {
            2 => "2",
            1 => "1",
            0 => "0",
            -1 => "-",
            -2 => "=",
            _ => panic!(),
        };

        number /= 5;
    }
    if carry == 1 {
        string.push('1');
    }

    string.chars().rev().collect::<String>()
}
