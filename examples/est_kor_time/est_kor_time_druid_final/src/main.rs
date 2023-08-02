#![windows_subsystem = "windows"]

use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc};

use chrono::Utc;
use chrono_tz::America::New_York;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
// const TEXT_BOX_WIDTH: f64 = 200.0;

fn get_curent_time_est() -> String {
    let utc = Utc::now();
    let ny_time = utc.with_timezone(&New_York);
    return format!(" {}", ny_time);
}

#[derive(Clone, Data, Lens)]
struct TimeState {
    time: String,
}

fn build_root_widget() -> impl Widget<TimeState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &TimeState, _env: &Env| {
        if data.time.is_empty() {
            get_curent_time_est()
        } else {
            format!("Hello {}!", data.time)
        }
    })
    .with_text_size(32.0);

    // a textbox that modifies `name`.
    // let textbox = TextBox::new()
    //     .with_placeholder("Who are we greeting?")
    //     .with_text_size(18.0)
    //     .fix_width(TEXT_BOX_WIDTH)
    //     .lens(TimeState ::name);

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        // .with_child(textbox)
        .align_vertical(UnitPoint::CENTER)
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("NewYork_Time: EST!")
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state: TimeState = TimeState {
        time: "".to_string(),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}
