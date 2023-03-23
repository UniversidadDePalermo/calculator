use druid::{Data, Lens};

/// Sets the mode for the numeric representation
#[derive(Clone, Data, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    Binary,
    Octal,
    #[default]
    Decimal,
    Hexa,
}

#[derive(Clone, Data, Lens)]
pub struct State {
    mode: Mode,
    value: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Mode::default(),
            value: String::default(),
        }
    }
}

impl State {
    /// Writes a digit to the internal value tracker
    pub fn write(&mut self, string: char) {
        self.value.push_str(string.to_string().as_str());
    }

    /// Clears internal value tracker
    pub fn clear(&mut self) {
        self.value = String::default();
    }

    /// Set different mode
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

#[cfg(test)]
mod tests {
    use super::{Mode, State};

    #[test]
    fn writes_digit_to_value() {
        let mut state = State::default();

        state.write('1');

        assert_eq!(state.value, "1");
    }

    #[test]
    fn accumulates_values_from_left_to_right() {
        let mut state = State::default();

        state.write('1');
        state.write('2');
        state.write('3');

        assert_eq!(state.value, "123");
    }

    #[test]
    fn clears_internal_value_tracker() {
        let mut state = State::default();

        state.write('1');
        state.write('2');
        state.write('3');

        assert_eq!(state.value, "123");

        state.clear();

        assert!(state.value.is_empty());
        assert_eq!(state.value, "");
    }

    #[test]
    fn updates_internal_mode_reference() {
        let mut state = State::default();

        assert_eq!(state.mode, Mode::Decimal);

        state.set_mode(Mode::Hexa);

        assert_eq!(state.mode, Mode::Hexa);
    }
}
