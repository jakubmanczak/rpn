#[derive(Debug, PartialEq)]
pub enum EvaluationResult {
    Success(i32),
    InputEmpty,
    InputNotComplete,
    InvalidCharacterFound(char),
    FoundNonOperator(char),
    FoundNonDigit(char),
    InputNumberOverflow,
    Overflow {
        last_valid_value1: i32,
        last_valid_value2: i32,
        attempted_operation: char,
    },
    Underflow {
        last_valid_value1: i32,
        last_valid_value2: i32,
        attempted_operation: char,
    },
}

#[derive(Debug, PartialEq)]
enum EvaluationStep {
    ReadingValue1,
    ReadingValue2,
    ReadingOperator,
}

impl EvaluationStep {
    pub fn advance(&mut self) {
        use EvaluationStep as S;
        *self = match self {
            S::ReadingValue1 => S::ReadingValue2,
            S::ReadingValue2 => S::ReadingOperator,
            S::ReadingOperator => S::ReadingValue2,
        };
    }
}

pub trait RpnOperator {
    fn is_valid_rpn_operator(&self) -> bool;
}

impl RpnOperator for char {
    fn is_valid_rpn_operator(&self) -> bool {
        match self {
            '+' | '-' => true,
            _ => false,
        }
    }
}

pub fn evaluate_rpn(input: &str) -> EvaluationResult {
    use EvaluationResult as ER;
    use EvaluationStep as S;
    let input = input.trim();

    let mut value1: i32 = 0;
    let mut value2: i32 = 0;
    let mut step = S::ReadingValue1;

    for c in input.chars() {
        match c {
            ' ' => {
                step.advance();
                if step == S::ReadingValue2 {
                    value2 = 0;
                }
            }
            c if c.is_ascii_digit() => match step {
                S::ReadingValue1 => {
                    let digit = c.to_digit(10).unwrap() as i32;
                    value1 = match value1.checked_mul(10) {
                        Some(value1) => match value1.checked_add(digit) {
                            Some(value1) => value1,
                            None => return ER::InputNumberOverflow,
                        },
                        None => return ER::InputNumberOverflow,
                    };
                }
                S::ReadingValue2 => {
                    let digit = c.to_digit(10).unwrap() as i32;
                    value2 = match value2.checked_mul(10) {
                        Some(value2) => match value2.checked_add(digit) {
                            Some(value2) => value2,
                            None => return ER::InputNumberOverflow,
                        },
                        None => return ER::InputNumberOverflow,
                    };
                }
                S::ReadingOperator => return ER::FoundNonOperator(c),
            },
            c if c.is_valid_rpn_operator() => match step {
                S::ReadingOperator => match c {
                    '+' => {
                        value1 = match value1.checked_add(value2) {
                            Some(value1) => value1,
                            None => {
                                return ER::Overflow {
                                    last_valid_value1: value1,
                                    last_valid_value2: value2,
                                    attempted_operation: c,
                                }
                            }
                        }
                    }
                    '-' => {
                        value1 = match value1.checked_sub(value2) {
                            Some(value2) => value2,
                            None => {
                                return ER::Underflow {
                                    last_valid_value1: value1,
                                    last_valid_value2: value2,
                                    attempted_operation: c,
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                S::ReadingValue1 | S::ReadingValue2 => return ER::FoundNonDigit(c),
            },
            invalid => return ER::InvalidCharacterFound(invalid),
        }
    }

    ER::Success(value1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use EvaluationResult as ER;

    #[test]
    fn no_operators_one() {
        let input = "1";
        assert_eq!(evaluate_rpn(input), ER::Success(1));
    }

    #[test]
    fn no_operators_forty_two() {
        let input = "42";
        assert_eq!(evaluate_rpn(input), ER::Success(42));
    }

    #[test]
    fn no_operators_two_thousand_and_four() {
        let input = "2004";
        assert_eq!(evaluate_rpn(input), ER::Success(2004));
    }

    #[test]
    fn one_plus_one() {
        let input = "1 1 +";
        assert_eq!(evaluate_rpn(input), ER::Success(2));
    }

    #[test]
    fn two_plus_two() {
        let input = "2 2 +";
        assert_eq!(evaluate_rpn(input), ER::Success(4));
    }

    #[test]
    fn forty_two_plus_forty_two() {
        let input = "42 42 +";
        assert_eq!(evaluate_rpn(input), ER::Success(84));
    }

    #[test]
    fn thousand_plus_sixty_nine() {
        let input = "1000 69 +";
        assert_eq!(evaluate_rpn(input), ER::Success(1069));
    }

    #[test]
    fn one_minus_one() {
        let input = "1 1 -";
        assert_eq!(evaluate_rpn(input), ER::Success(0));
    }

    #[test]
    fn thousand_minus_one() {
        let input = "1000 1 -";
        assert_eq!(evaluate_rpn(input), ER::Success(999));
    }

    #[test]
    fn getting_a_negative_num() {
        let input = "344 1454364 -";
        assert_eq!(evaluate_rpn(input), ER::Success(-1_454_020));
    }

    #[test]
    fn five_plus_five_plus_five() {
        let input = "5 5 + 5 +";
        assert_eq!(evaluate_rpn(input), ER::Success(15));
    }

    #[test]
    fn complex_addition_and_subtraction() {
        let input = "2 3 + 4 + 5 + 144 - 45 + 3 + 3434 - 34 - 34 - 3455 + 129 +";
        assert_eq!(evaluate_rpn(input), ER::Success(0));
    }
}
