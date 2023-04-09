use std::fmt::Display;

use druid::{Data, Lens};
use thiserror::Error;

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
    base: Base,
    /// Represents the `inner_value` in Base-N where `N` could be
    /// either Binary, Octal, Decimal or Hexa.
    value: String,
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
    pub fn write(&mut self, string: char) -> Result<()> {
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
                    self.value.push(string);
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
    pub fn set_base(&mut self, next_base: Base) {
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

        self.base = next_base;
    }
}

#[cfg(test)]
mod tests {
    use super::{Base, State};

    #[test]
    fn writes_digit_to_value() {
        let mut state = State::default();

        state.write('1').unwrap();

        assert_eq!(state.value, "1");
    }

    #[test]
    fn accumulates_values_from_left_to_right() {
        let mut state = State::default();

        state.write('1').unwrap();
        state.write('2').unwrap();
        state.write('3').unwrap();

        assert_eq!(state.value, "123");
    }

    #[test]
    fn clears_internal_value_tracker() {
        let mut state = State::default();

        state.write('1').unwrap();
        state.write('2').unwrap();
        state.write('3').unwrap();

        assert_eq!(state.value, "123");

        state.clear();

        assert!(state.value.is_empty());
        assert_eq!(state.value, "");
    }

    #[test]
    fn updates_internal_base_reference() {
        let mut state = State::default();

        assert_eq!(state.base, Base::Decimal);

        state.set_base(Base::Hexa);

        assert_eq!(state.base, Base::Hexa);
    }

    #[test]
    fn changes_base_from_dec_to_bin() {
        let mut state = State::default();

        assert_eq!(state.base, Base::Decimal);

        state.write('2').unwrap();
        state.write('7').unwrap();

        state.set_base(Base::Binary);

        assert_eq!(state.base, Base::Binary);
        assert_eq!(state.value, String::from("11011"));
    }

    #[test]
    fn changes_base_from_bin_to_dec() {
        let mut state = State::default();

        state.set_base(Base::Binary);

        state.write('1').unwrap();
        state.write('1').unwrap();
        state.write('0').unwrap();
        state.write('1').unwrap();
        state.write('1').unwrap();

        state.set_base(Base::Decimal);

        assert_eq!(state.base, Base::Decimal);
        assert_eq!(state.value, String::from("27"));
    }

    #[test]
    fn changes_base_from_oct_to_dec() {
        let mut state = State::default();

        state.set_base(Base::Octal);

        state.write('1').unwrap();
        state.write('7').unwrap();
        state.write('0').unwrap();
        state.write('3').unwrap();
        state.write('4').unwrap();

        state.set_base(Base::Decimal);

        assert_eq!(state.base, Base::Decimal);
        assert_eq!(state.value, String::from("7708"));
    }

    #[test]
    fn changes_base_from_dec_to_oct() {
        let mut state = State::default();

        state.set_base(Base::Decimal);

        state.write('7').unwrap();
        state.write('7').unwrap();
        state.write('0').unwrap();
        state.write('8').unwrap();

        state.set_base(Base::Octal);

        assert_eq!(state.base, Base::Octal);
        assert_eq!(state.value, String::from("17034"));
    }

    #[test]
    fn changes_base_from_dec_to_hex() {
        let mut state = State::default();

        state.set_base(Base::Decimal);

        state.write('1').unwrap();
        state.write('7').unwrap();
        state.write('0').unwrap();
        state.write('8').unwrap();
        state.write('2').unwrap();
        state.write('8').unwrap();
        state.write('8').unwrap();
        state.write('2').unwrap();
        state.write('9').unwrap();

        state.set_base(Base::Hexa);

        assert_eq!(state.base, Base::Hexa);
        assert_eq!(state.value, String::from("A2EA41D"));
    }

    #[test]
    fn changes_base_from_hex_to_dec() {
        let mut state = State::default();

        state.set_base(Base::Hexa);

        state.write('A').unwrap();
        state.write('2').unwrap();
        state.write('E').unwrap();
        state.write('A').unwrap();
        state.write('4').unwrap();
        state.write('1').unwrap();
        state.write('D').unwrap();

        state.set_base(Base::Decimal);

        assert_eq!(state.base, Base::Decimal);
        assert_eq!(state.value, String::from("170828829"));
    }
}
