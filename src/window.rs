use druid::widget::{Flex, FlexParams, Label};
use druid::{Widget, WidgetExt};

use crate::state::State;

pub struct Window;

impl Window {
    pub fn build() -> impl Widget<State> {
        let display = Label::new(|data: &String, _env: &_| data.clone())
            .with_text_size(32.0)
            .lens(State::value)
            .padding(5.0);

        Flex::column().with_flex_spacer(0.2).with_child(display)
    }
}
