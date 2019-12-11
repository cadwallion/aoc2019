fn digits_never_decrease(number: i32) -> bool {
    let digit1 = number % 10;
    let digit2 = number / 10 % 10;
    if (number / 10) == 0 {
        true
    } else if digit2 <= digit1 {
        digits_never_decrease(number / 10)
    } else {
        false
    }
}

fn has_two_adjacent_digits(number: i32) -> bool {
    let mut found = 0;
    let mut number = number.clone();
    while number / 10 != 0 {
        let digit1 = number % 10;
        let digit2 = number / 10 % 10;

        if digit1 == digit2 {
            found += 1;
        }
        number /= 10;
    }
    found > 0
}
// 6-digits
// within 246540 and 787419
// two digits identical
// L2R digits never decrease (but they can stay the same)
pub fn a() {
    let mut valid_answers = Vec::<i32>::new();

    for number in 246540..=787419 {
        if digits_never_decrease(number) && has_two_adjacent_digits(number) {
            valid_answers.push(number);
        }
    }

    println!("Day 4a: {}", valid_answers.len());
}
