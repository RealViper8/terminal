use std::iter::Peekable;
use std::str::Chars;

pub struct MathInterpreter;

impl MathInterpreter {
    pub fn new() -> Self {
        MathInterpreter
    }

    pub fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        let mut chars = expression.chars().peekable();
        self.parse_expression(&mut chars)
    }

    fn parse_expression(&mut self, chars: &mut Peekable<Chars>) -> Result<f64, String> {
        self.parse_term(chars)
    }

    fn parse_term(&mut self, chars: &mut Peekable<Chars>) -> Result<f64, String> {
        let mut value = self.parse_factor(chars)?;

        while let Some(&next) = chars.peek() {
            match next {
                '+' => {
                    chars.next();
                    value += self.parse_factor(chars)?;
                }
                '-' => {
                    chars.next();
                    value -= self.parse_factor(chars)?;
                }
                _ => break,
            }
        }

        Ok(value)
    }

    fn parse_factor(&mut self, chars: &mut Peekable<Chars>) -> Result<f64, String> {
        let mut value = self.parse_primary(chars)?;

        while let Some(&next) = chars.peek() {
            match next {
                '*' => {
                    chars.next();
                    value *= self.parse_primary(chars)?;
                }
                '/' => {
                    chars.next();
                    let divisor = self.parse_primary(chars)?;
                    if divisor == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    value /= divisor;
                }
                _ => break,
            }
        }

        Ok(value)
    }

    fn parse_primary(&mut self, chars: &mut Peekable<Chars>) -> Result<f64, String> {
        if let Some(&next) = chars.peek() {
            match next {
                '0'..='9' | '.' => self.parse_number(chars),
                '(' => {
                    chars.next(); // consume '('
                    let value = self.parse_expression(chars)?;
                    if chars.next() == Some(')') {
                        Ok(value)
                    } else {
                        Err("Expected closing parenthesis".to_string())
                    }
                }
                _ => Err("Invalid character".to_string()),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn parse_number(&mut self, chars: &mut Peekable<Chars>) -> Result<f64, String> {
        let mut number_str = String::new();
        while let Some(&next) = chars.peek() {
            if next.is_ascii_digit() || next == '.' {
                number_str.push(chars.next().unwrap());
            } else {
                break;
            }
        }

        number_str
            .parse::<f64>()
            .map_err(|_| "Invalid number".to_string())
    }
}
