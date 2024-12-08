fn input() -> &'static str {
    include_str!("../inputs/1.txt")
}

// Primary objective: parsing the input in a single pass, only touching each character once.
// Due to the way the puzzle is designed, this is achievable just by storing the last token

enum Token {
    M,
    U,
    L,
    D,
    O,
    N,
    Hyphen,
    T,
    ParenOpen,
    ParenClose,
    Comma,
    Number(String)
}

enum Function {
    Multiply(i32, i32),
    Do,
    Dont,
}

fn get_next_token(last_token: &Option<Token>, character: char) -> Option<Token> {
    match character {
        'm' => { Some(Token::M) },
        'u' => { Some(Token::U) },
        'l' => { Some(Token::L) },
        'd' => { Some(Token::D) },
        'o' => { Some(Token::O) },
        'n' => { Some(Token::N) },
        '\'' => { Some(Token::Hyphen) },
        't' => { Some(Token::T) },
        '(' => { Some(Token::ParenOpen) },
        ')' => { Some(Token::ParenClose) },
        ',' => { Some(Token::Comma) },
        '0'..='9' => {
            match last_token {
                Some(Token::Number(digits)) => {
                    Some(Token::Number(format!("{}{}", digits.clone(), character)))
                },
                _ => {
                    Some(Token::Number(character.to_string()))
                }
            }
        },
        _ => {
            None
        }
    }
}

fn tokenize(input: &str) -> Vec<Function> {
    let mut function_calls: Vec<Function> = Vec::new();
    let mut current_function_call: Option<Function> = None;
    let mut last_token: Option<Token> = None;
    for character in input.chars() {
        let token = get_next_token(&last_token, character);
        let valid_token = match token {
            Some(Token::M) => { token },
            Some(Token::U) => { if matches!(last_token, Some(Token::M)) { token } else { None } },
            Some(Token::L) => { if matches!(last_token, Some(Token::U)) { token } else { None }  },
            Some(Token::D) => { token },
            Some(Token::O) => { if matches!(last_token, Some(Token::D)) { token } else { None } },
            Some(Token::N) => { if matches!(last_token, Some(Token::O)) { token } else { None } },
            Some(Token::Hyphen) => { if matches!(last_token, Some(Token::N)) { token } else { None } },
            Some(Token::T) => { if matches!(last_token, Some(Token::Hyphen)) { token } else { None } },
            Some(Token::ParenOpen) => {
                match last_token {
                    Some(Token::L) => {
                        current_function_call = Some(Function::Multiply(0, 0));
                        token
                    },
                    Some(Token::O) => {
                        current_function_call = Some(Function::Do);
                        token
                    },
                    Some(Token::T) => {
                        current_function_call = Some(Function::Dont);
                        token
                    },
                    _ => None
                }
            },
            Some(Token::ParenClose) => {
                match current_function_call {
                    Some(Function::Multiply(left_operand, _)) => {
                        match last_token {
                            Some(Token::Number(digits)) => {
                                function_calls.push(Function::Multiply(left_operand, digits.parse().unwrap()));
                                current_function_call = None;
                                token
                            },
                            _ => None
                        }
                    },
                    Some(Function::Do) => {
                        let Some(Token::ParenOpen) = last_token else {
                            unreachable!("Expected no parameters for do");
                        };
                        function_calls.push(Function::Do);
                        current_function_call = None;
                        token
                    },
                    Some(Function::Dont) => {
                        let Some(Token::ParenOpen) = last_token else {
                            unreachable!("Expected no parameters for don't");
                        };
                        function_calls.push(Function::Dont);
                        current_function_call = None;
                        token
                    }
                    _ => None
                }
            },
            Some(Token::Comma) => { if matches!(last_token, Some(Token::Number(_))) {
                match last_token {
                    Some(Token::Number(digits)) => {
                        match current_function_call {
                            Some(Function::Multiply(_, _)) => {
                                current_function_call = Some(Function::Multiply(digits.parse().unwrap(), 0));
                                token
                            },
                            _ => None
                        }
                    },
                    _ => None
                }
            } else { None } },
            Some(Token::Number(digits)) => {
                if digits.len() <= 3 {
                    Some(Token::Number(digits))
                } else {
                    None
                }
            },
            _ => None
        };
        if valid_token.is_none() {
            current_function_call = None;
        }
        last_token = valid_token;
    }
    function_calls
}

fn day3_1() -> i32 {
    let function_calls = tokenize(input());
    function_calls.iter().fold(0, |acc, function_call|
        match function_call {
            Function::Multiply(left_operand, right_operand) => acc + left_operand * right_operand,
            _ => acc
        }
    )
}

fn day3_2() -> i32 {
    let function_calls = tokenize(input());
    let mut toggle = true;
    function_calls.iter().fold(0, |acc, function_call|
        match function_call {
            Function::Multiply(left_operand, right_operand) => { if toggle { acc + left_operand * right_operand} else { acc } },
            Function::Do => { toggle = true; acc },
            Function::Dont => { toggle = false; acc }
        }
    )
}

fn main() {
    assert_eq!(day3_1(), 173517243);
    assert_eq!(day3_2(), 100450138);
}
