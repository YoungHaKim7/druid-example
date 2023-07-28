use druid::widget::{Button, Container, Flex, Label, LensWrap, List, Split, TextBox};
use druid::{AppLauncher, Color, Data, Lens, Widget, WindowDesc};
use im::{vector, Vector};

#[derive(Clone, Data, Lens)]
struct TodoList {
    items: Vector<String>,
    next_item: String,
}

fn build_ui() -> impl Widget<TodoList> {
    Split::columns(
        Container::new(LensWrap::new(
            List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
            TodoList::items,
        ))
        .border(Color::grey(0.6), 2.0),
        Container::new(
            Flex::column()
                .with_flex_child(
                    Button::new("Add item").on_click(|_, data: &mut TodoList, _| {
                        data.items.push_back(data.next_item.clone());
                        data.next_item = String::new();
                    }),
                    1.0,
                )
                .with_flex_child(LensWrap::new(TextBox::new(), TodoList::next_item), 1.0),
        )
        .border(Color::grey(0.6), 3.0),
    )
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My first Druid App");
    let initial_data = TodoList {
        items: vector![
            "first item".into(),
            "second item".into(),
            "third item".into(),
            "foo".into(),
            "bar".into(),
        ],
        next_item: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
