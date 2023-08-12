use data::TodoState;
use druid::{
    theme::{self, BUTTON_DARK, BUTTON_LIGHT, WINDOW_BACKGROUND_COLOR},
    AppLauncher, Color, FontDescriptor, WindowDesc,
};
use im::Vector;
use saver::read_stored;
use ui::ui_builder;

mod data;
mod saver;
mod ui;

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("My Todo App")
        .window_size((800., 800.))
        .resizable(true);

    let stored = read_stored();
    let default_state = TodoState {
        todos: Vector::from(stored.tasks),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, _state| {
            env.set(theme::UI_FONT, FontDescriptor::default().with_size(30.0));
            env.set(BUTTON_DARK, Color::rgba8(100, 100, 120, 0));
            env.set(BUTTON_LIGHT, Color::rgba8(100, 100, 120, 100));
            env.set(WINDOW_BACKGROUND_COLOR, Color::rgba8(0, 0, 0, 100));
        })
        .launch(default_state)
        .expect("failed app")
}
