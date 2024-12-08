use itertools::Itertools;

fn input() -> &'static str {
    include_str!("../inputs/1.txt")
}

enum Operator {
    Add,
    Multiply,
    Concatenate,
}

struct Equation<'a> {
    values: &'a Vec<u32>,
    operators: &'a Vec<&'a Operator>,
    expected_result: u64,
}

impl Equation<'_> {
    fn new<'a>(values: &'a Vec<u32>, operators: &'a Vec<&'a Operator>, expected_result: u64) -> Equation<'a> {
        if values.len() != operators.len() + 1 {
            panic!("Invalid equation");
        }
        Equation {
            values,
            operators,
            expected_result,
        }
    }
}

fn for_each_operator_permutation<F>(value_count: usize, allowed_operators: &[Operator], predicate: F)
where F: FnMut(Vec<&Operator>) -> bool {
    (1..value_count)
        .map(|_| allowed_operators.iter())
        .multi_cartesian_product()
        .all(predicate);
}

fn any_operator_permutation<F>(value_count: usize, allowed_operators: &[Operator], mut predicate: F) -> bool
where F: FnMut(Vec<&Operator>) -> bool {
    let mut true_for_any = false;
    for_each_operator_permutation(value_count, allowed_operators, |operator_permutation| {
        if predicate(operator_permutation) {
            true_for_any = true;
            false
        } else {
            true
        }
    });
    true_for_any
}

fn concatenate_numbers(a: u64, b: u64) -> u64 {
    let mut result = a;
    let mut temp = b;
    while temp > 0 {
        result *= 10;
        temp /= 10;
    }
    result + b
}

fn calculate_equation(equation: &Equation) -> u64 {
    equation.values.iter().skip(1).enumerate().fold(equation.values[0] as u64, |sum, (index, value)| {
        match equation.operators[index] {
            Operator::Add => sum + *value as u64,
            Operator::Multiply => sum * *value as u64,
            Operator::Concatenate => concatenate_numbers(sum, *value as u64),
        }
    })
}

fn is_valid_equation(equation: &Equation) -> bool {
    calculate_equation(equation) == equation.expected_result
}

fn has_valid_operator_permutation(values: &Vec<u32>, allowed_operators: &[Operator], expected_result: u64) -> bool {
    any_operator_permutation(values.len(), allowed_operators, |operator_permutation| {
        let equation = Equation::new(values, &operator_permutation, expected_result);
        is_valid_equation(&equation)
    })
}

fn for_each_input_lines<F>(mut predicate: F)
where F: FnMut(u64, &Vec<u32>){
    input().lines().for_each(|line| {
        let mut split = line.split(": ");
        let expected_result = split.next()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let values: Vec<_> = split.next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|value| value.parse::<u32>().unwrap())
            .collect();
        predicate(expected_result, &values);
    });
}

fn day7_1() -> u64 {
    let mut sum = 0;
    let allowed_operators = [Operator::Add, Operator::Multiply];
    for_each_input_lines(|expected_result, values| {
        if has_valid_operator_permutation(values, &allowed_operators, expected_result) {
            sum += expected_result;
        }
    });
    sum
}

fn day7_2() -> u64 {
    let mut sum = 0;
    let allowed_operators = [Operator::Add, Operator::Multiply, Operator::Concatenate];
    for_each_input_lines(|expected_result, values| {
        if has_valid_operator_permutation(values, &allowed_operators, expected_result) {
            sum += expected_result;
        }
    });
    sum
}

fn main() {
    assert_eq!(day7_1(), 465126289353);
    assert_eq!(day7_2(), 70597497486371);
}
