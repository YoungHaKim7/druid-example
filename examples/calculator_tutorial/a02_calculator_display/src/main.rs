use druid::{
    theme,
    widget::{CrossAxisAlignment, Flex, Label, Painter},
    AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct CalcState {
    value: String,
}

impl CalcState {
    fn display(&mut self) {
        self.value.to_string();
    }
}

fn op_button_label(op: char, label: String) -> impl Widget<CalcState> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0)
        };

        if ctx.is_active() {
            ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
        }
    });

    Label::new(label)
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
}

fn op_button(op: char) -> impl Widget<CalcState> {
    op_button_label(op, op.to_string())
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
    w3: impl Widget<T> + 'static,
    w4: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w3, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w4, 1.0)
}

fn build_calc() -> impl Widget<CalcState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(32.0)
        .lens(CalcState::value)
        .padding(5.0);
    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(
            flex_row(
                op_button_label('c', "CE".to_string()),
                op_button('C'),
                op_button('⌫'),
                op_button('÷'),
            ),
            1.0,
        )
        .with_spacer(1.0)
}

pub fn main() {
    let window = WindowDesc::new(build_calc())
        .window_size((223., 300.))
        .resizable(true)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
        );
    let calc_state = CalcState {
        value: "0".to_string(),
    };
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(calc_state)
        .expect("launch failed");
}
