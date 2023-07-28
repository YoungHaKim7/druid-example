use druid::widget::{Button, Container, Flex, Label, List, Split};
use druid::{AppLauncher, Color, Widget, WindowDesc};
use im::{vector, Vector};

type TodoList = Vector<String>;

fn build_ui() -> impl Widget<TodoList> {
    Split::columns(
        Container::new(
            // Dynamic list of Widgets
            List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
        )
        .border(Color::grey(0.6), 2.0),
        Container::new(
            Flex::column()
                .with_flex_child(Button::new("Button placeholder"), 1.0)
                .with_flex_child(Label::new("Textbox placeholder"), 1.0),
        )
        .border(Color::grey(0.6), 2.0),
    )
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My first Druid App");
    let initial_data = vector![
        "first item".into(),
        "second item".into(),
        "third item".into(),
        "foo".into(),
        "bar".into(),
    ];

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
