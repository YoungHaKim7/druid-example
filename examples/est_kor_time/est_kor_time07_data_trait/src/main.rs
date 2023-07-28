use druid::{
    widget::{Button, Container, Flex, Label, LensWrap, List, Split},
    AppLauncher, Color, Data, Widget, WindowDesc,
};
use im::vector;
use std::sync::Arc;

#[derive(Clone, Data)]
/// The main model for a todo list application.
struct TodoList {
    items: Arc<Vec<TodoItem>>,
}

#[derive(Clone, Data)]
/// A single todo item.
struct TodoItem {
    category: Category,
    title: String,
    note: Option<String>,
    completed: bool,

    // `Data` is implemented for any `Arc`.
    due_date: Option<Arc<DateTime>>,

    // You can specify a custom comparison fn
    // (anything with the signature (&T, &T) -> bool).
    #[data(same_fn = "PartialEq::eq")]
    added_date: DateTime,

    // You can specify that a field should
    // be skipped when computing same-ness
    #[data(ignore)]
    debug_timestamp: usize,
}

#[derive(Clone, Data, PartialEq)]
/// The three types of tasks in the world.
enum Category {
    Work,
    Play,
    Revolution,
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
