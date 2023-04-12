use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};
use druid::{Color, RenderContext, Widget, WidgetExt};

use crate::state::{Base, State, WriteValue};

pub struct Window;

impl Window {
    pub fn build() -> impl Widget<State> {
        const SPACE: f64 = 1.0;

        let display = Label::new(|data: &String, _env: &_| data.clone())
            .with_text_size(32.0)
            .lens(State::value)
            .padding(5.0);

        Flex::column()
            .with_flex_spacer(0.2)
            .with_child(display)
            .with_flex_spacer(0.2)
            .cross_axis_alignment(CrossAxisAlignment::End)
            .with_flex_child(
                // First Button Row
                Flex::row()
                    .with_flex_child(Self::base_mode_button(Base::Binary), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::DigitD), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::DigitE), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::DigitF), SPACE),
                SPACE,
            )
            .with_flex_child(
                // Second Button Row
                Flex::row()
                    .with_flex_child(Self::base_mode_button(Base::Octal), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::DigitA), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::DigitB), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::DigitC), SPACE),
                SPACE,
            )
            .with_flex_child(
                // Third Button Row
                Flex::row()
                    .with_flex_child(Self::base_mode_button(Base::Decimal), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::Digit7), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::Digit8), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::Digit9), SPACE),
                SPACE,
            )
            .with_flex_child(
                // Fourth Button Row
                Flex::row()
                    .with_flex_child(Self::base_mode_button(Base::Hexa), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::Digit4), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::Digit5), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::Digit6), SPACE),
                SPACE,
            )
            .with_flex_child(
                // Fifth Button Row
                Flex::row()
                    .with_flex_child(Self::write_button(WriteValue::Digit0), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::Digit1), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::Digit2), SPACE)
                    .with_flex_child(Self::write_button(WriteValue::Digit3), SPACE)
                    .with_flex_child(Self::clear_button(), SPACE),
                    
                SPACE,
            )
    }

    fn clear_button() -> impl Widget<State> {
        let background_config = Painter::new(|ctx: _, _: _, _: _| {
            let bounds = ctx.size().to_rect();

            ctx.fill(bounds, &Color::grey(0.9));

            if ctx.is_hot() {
                ctx.stroke(bounds.inset(-0.5), &Color::grey(0.6), 1.0);
            }

            if ctx.is_active() {
                ctx.fill(bounds, &Color::grey(1.0));
            }
        });

        Label::new(String::from("AC"))
            .with_text_color(Color::BLACK)
            .with_text_size(24.)
            .center()
            .background(background_config)
            .expand()
            .on_click(move |_ctx, data: &mut State, _env| data.clear())
    }

    /// Creates a Button to Write digits
    fn write_button(value: WriteValue) -> impl Widget<State> {
        let background_config = Painter::new(|ctx: _, _: _, _: _| {
            let bounds = ctx.size().to_rect();

            ctx.fill(bounds, &Color::grey(0.9));

            if ctx.is_hot() {
                ctx.stroke(bounds.inset(-0.5), &Color::grey(0.6), 1.0);
            }

            if ctx.is_active() {
                ctx.fill(bounds, &Color::grey(1.0));
            }
        });

        let value_clone = value.clone();

        Label::new(String::from(value.into_char()))
            .with_text_color(Color::BLACK)
            .with_text_size(24.)
            .center()
            .disabled_if(move |data: &State, _| !value_clone.is_allowed_for_base(data.base))
            .background(background_config)
            .expand()
            .on_click(move |_ctx, data: &mut State, _env| data.write(&value).unwrap())
    }

    /// Creates a Button to Write digits
    fn base_mode_button(base: Base) -> impl Widget<State> {
        let text = match base {
            Base::Binary => "Bin",
            Base::Octal => "Oct",
            Base::Decimal => "Dec",
            Base::Hexa => "Hex",
        };

        let background_config = Painter::new(|ctx: _, _: _, _: _| {
            let bounds = ctx.size().to_rect();

            ctx.fill(bounds, &Color::grey(0.9));

            if ctx.is_hot() {
                ctx.stroke(bounds.inset(-0.5), &Color::grey(0.6), 1.0);
            }

            if ctx.is_active() {
                ctx.fill(bounds, &Color::grey(1.0));
            }
        });

        Label::new(text)
            .with_text_color(Color::BLACK)
            .with_text_size(24.)
            .center()
            .background(background_config)
            .expand()
            .on_click(move |_ctx, data: &mut State, _env| data.set_base(&base))
    }
}
