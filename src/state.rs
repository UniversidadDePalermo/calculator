use std::fmt::Display;

use druid::{Data, Lens};
use thiserror::Error;

/// Writeable Values
#[derive(Clone, Debug)]
pub enum WriteValue {
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    DigitA,
    DigitB,
    DigitC,
    DigitD,
    DigitE,
    DigitF,
}

impl WriteValue {
    /// Returs the character representation of the `WriteValue`
    pub fn into_char(&self) -> char {
        match self {
            WriteValue::Digit0 => '0',
            WriteValue::Digit1 => '1',
            WriteValue::Digit2 => '2',
            WriteValue::Digit3 => '3',
            WriteValue::Digit4 => '4',
            WriteValue::Digit5 => '5',
            WriteValue::Digit6 => '6',
            WriteValue::Digit7 => '7',
            WriteValue::Digit8 => '8',
            WriteValue::Digit9 => '9',
            WriteValue::DigitA => 'A',
            WriteValue::DigitB => 'B',
            WriteValue::DigitC => 'C',
            WriteValue::DigitD => 'D',
            WriteValue::DigitE => 'E',
            WriteValue::DigitF => 'F',
        }
    }

    /// Checks if the Digit is valid for the provided `Base`
    pub fn is_allowed_for_base(&self, b: Base) -> bool {
        match b {
            Base::Binary => match self {
                WriteValue::Digit0 | WriteValue::Digit1 => true,
                _ => false,
            },
            Base::Octal => match self {
                WriteValue::Digit0
                | WriteValue::Digit1
                | WriteValue::Digit2
                | WriteValue::Digit3
                | WriteValue::Digit4
                | WriteValue::Digit5
                | WriteValue::Digit6
                | WriteValue::Digit7 => true,
                _ => false,
            },
            Base::Decimal => match self {
                WriteValue::Digit0
                | WriteValue::Digit1
                | WriteValue::Digit2
                | WriteValue::Digit3
                | WriteValue::Digit4
                | WriteValue::Digit5
                | WriteValue::Digit6
                | WriteValue::Digit7
                | WriteValue::Digit8
                | WriteValue::Digit9 => true,
                _ => false,
            },
            Base::Hexa => match self {
                WriteValue::Digit0
                | WriteValue::Digit1
                | WriteValue::Digit2
                | WriteValue::Digit3
                | WriteValue::Digit4
                | WriteValue::Digit5
                | WriteValue::Digit6
                | WriteValue::Digit7
                | WriteValue::Digit8
                | WriteValue::Digit9
                | WriteValue::DigitA
                | WriteValue::DigitB
                | WriteValue::DigitC
                | WriteValue::DigitD
                | WriteValue::DigitE
                | WriteValue::DigitF => true,
            },
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum StateError {
    #[error("Invalid character provided {0}. Current base is {1}")]
    InvalidChar(char, Base),
}

pub type Result<T> = std::result::Result<T, StateError>;

/// Sets the mode for the numeric representation
#[derive(Copy, Clone, Data, Debug, Default, PartialEq, Eq)]
pub enum Base {
    Binary,
    Octal,
    #[default]
    Decimal,
    Hexa,
}

impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base::Binary => write!(f, "Binary"),
            Base::Octal => write!(f, "Octal"),
            Base::Decimal => write!(f, "Decimal"),
            Base::Hexa => write!(f, "Hexa"),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct State {
    /// Specifies the current base
    pub(super) base: Base,
    /// Represents the `inner_value` in Base-N where `N` could be
    /// either Binary, Octal, Decimal or Hexa.
    pub(super) value: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            base: Base::default(),
            value: String::default(),
        }
    }
}

impl State {
    /// Writes a digit to the internal value tracker
    pub fn write(&mut self, value: &WriteValue) -> Result<()> {
        let string = value.into_char();

        if self.value == "0" {
            self.value = String::default();
        }

        match self.base {
            Base::Binary => {
                if string == '0' || string == '1' {
                    self.value.push(string);
                    return Ok(());
                }

                Err(StateError::InvalidChar(string, self.base))
            }
            Base::Octal => {
                if string >= '0' && string <= '7' {
                    self.value.push(string);
                    return Ok(());
                }

                Err(StateError::InvalidChar(string, self.base))
            }
            Base::Decimal => {
                if string >= '0' && string <= '9' {
                    self.value.push(string);
                    return Ok(());
                }

                Err(StateError::InvalidChar(string, self.base))
            }
            Base::Hexa => {
                let string = string.to_ascii_lowercase();

                if string >= '0' && string <= '9' || string >= 'a' && string <= 'f' {
                    self.value.push(string.to_ascii_uppercase());
                    return Ok(());
                }

                Err(StateError::InvalidChar(string, self.base))
            }
        }
    }

    /// Clears internal value tracker
    pub fn clear(&mut self) {
        self.value = String::default();
    }

    fn value_base_10(&self) -> i128 {
        match self.base {
            Base::Binary => i128::from_str_radix(&self.value, 2).unwrap(),
            Base::Octal => i128::from_str_radix(&self.value, 8).unwrap(),
            Base::Decimal => i128::from_str_radix(&self.value, 10).unwrap(),
            Base::Hexa => i128::from_str_radix(&self.value, 16).unwrap(),
        }
    }

    /// Set different mode
    pub fn set_base(&mut self, next_base: &Base) {
        if self.value.is_empty() {
            // Cannot parse a int from an empty string
            self.value = String::from("0");
        }

        match next_base {
            Base::Binary => {
                let value = self.value_base_10();

                self.value = format!("{:b}", value);
            }
            Base::Octal => {
                let value = self.value_base_10();

                self.value = format!("{:o}", value);
            }
            Base::Decimal => {
                let value = self.value_base_10();

                self.value = format!("{}", value);
            }
            Base::Hexa => {
                let value = self.value_base_10();

                self.value = format!("{:x}", value).to_ascii_uppercase();
            }
        }

        self.base = next_base.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::{Base, State, WriteValue};

    #[test]
    fn writes_digit_to_value() {
        let mut state = State::default();

        state.write(&WriteValue::Digit1).unwrap();

        assert_eq!(state.value, "1");
    }

    #[test]
    fn accumulates_values_from_left_to_right() {
        let mut state = State::default();

        state.write(&WriteValue::Digit1).unwrap();
        state.write(&WriteValue::Digit2).unwrap();
        state.write(&WriteValue::Digit3).unwrap();

        assert_eq!(state.value, "123");
    }

    #[test]
    fn clears_internal_value_tracker() {
        let mut state = State::default();

        state.write(&WriteValue::Digit1).unwrap();
        state.write(&WriteValue::Digit2).unwrap();
        state.write(&WriteValue::Digit3).unwrap();

        assert_eq!(state.value, "123");

        state.clear();

        assert!(state.value.is_empty());
        assert_eq!(state.value, "");
    }

    #[test]
    fn updates_internal_base_reference() {
        let mut state = State::default();

        assert_eq!(state.base, Base::Decimal);

        state.set_base(&Base::Hexa);

        assert_eq!(state.base, Base::Hexa);
    }

    #[test]
    fn changes_base_from_dec_to_bin() {
        let mut state = State::default();

        assert_eq!(state.base, Base::Decimal);

        state.write(&WriteValue::Digit2).unwrap();
        state.write(&WriteValue::Digit7).unwrap();

        state.set_base(&Base::Binary);

        assert_eq!(state.base, Base::Binary);
        assert_eq!(state.value, String::from("11011"));
    }

    #[test]
    fn changes_base_from_bin_to_dec() {
        let mut state = State::default();

        state.set_base(&Base::Binary);

        state.write(&WriteValue::Digit1).unwrap();
        state.write(&WriteValue::Digit1).unwrap();
        state.write(&WriteValue::Digit0).unwrap();
        state.write(&WriteValue::Digit1).unwrap();
        state.write(&WriteValue::Digit1).unwrap();

        state.set_base(&Base::Decimal);

        assert_eq!(state.base, Base::Decimal);
        assert_eq!(state.value, String::from("27"));
    }

    #[test]
    fn changes_base_from_oct_to_dec() {
        let mut state = State::default();

        state.set_base(&Base::Octal);

        state.write(&WriteValue::Digit1).unwrap();
        state.write(&WriteValue::Digit7).unwrap();
        state.write(&WriteValue::Digit0).unwrap();
        state.write(&WriteValue::Digit3).unwrap();
        state.write(&WriteValue::Digit4).unwrap();

        state.set_base(&Base::Decimal);

        assert_eq!(state.base, Base::Decimal);
        assert_eq!(state.value, String::from("7708"));
    }

    #[test]
    fn changes_base_from_dec_to_oct() {
        let mut state = State::default();

        state.set_base(&Base::Decimal);

        state.write(&WriteValue::Digit7).unwrap();
        state.write(&WriteValue::Digit7).unwrap();
        state.write(&WriteValue::Digit0).unwrap();
        state.write(&WriteValue::Digit8).unwrap();

        state.set_base(&Base::Octal);

        assert_eq!(state.base, Base::Octal);
        assert_eq!(state.value, String::from("17034"));
    }

    #[test]
    fn changes_base_from_dec_to_hex() {
        let mut state = State::default();

        state.set_base(&Base::Decimal);

        state.write(&WriteValue::Digit1).unwrap();
        state.write(&WriteValue::Digit7).unwrap();
        state.write(&WriteValue::Digit0).unwrap();
        state.write(&WriteValue::Digit8).unwrap();
        state.write(&WriteValue::Digit2).unwrap();
        state.write(&WriteValue::Digit8).unwrap();
        state.write(&WriteValue::Digit8).unwrap();
        state.write(&WriteValue::Digit2).unwrap();
        state.write(&WriteValue::Digit9).unwrap();

        state.set_base(&Base::Hexa);

        assert_eq!(state.base, Base::Hexa);
        assert_eq!(state.value, String::from("A2EA41D"));
    }

    #[test]
    fn changes_base_from_hex_to_dec() {
        let mut state = State::default();

        state.set_base(&Base::Hexa);

        state.write(&WriteValue::DigitA).unwrap();
        state.write(&WriteValue::Digit2).unwrap();
        state.write(&WriteValue::DigitE).unwrap();
        state.write(&WriteValue::DigitA).unwrap();
        state.write(&WriteValue::Digit4).unwrap();
        state.write(&WriteValue::Digit1).unwrap();
        state.write(&WriteValue::DigitD).unwrap();

        state.set_base(&Base::Decimal);

        assert_eq!(state.base, Base::Decimal);
        assert_eq!(state.value, String::from("170828829"));
    }
}
