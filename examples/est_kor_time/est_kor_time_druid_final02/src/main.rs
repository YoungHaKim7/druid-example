#![windows_subsystem = "windows"]

use druid::widget::{CrossAxisAlignment, Controller};
use druid::widget::{Flex, Label};
use druid::{AppLauncher, Lens, WidgetExt, WindowDesc, Data, Widget, EventCtx, Event, Env};

use chrono::Utc;
use chrono_tz::America::New_York;

use druid::widget::ControllerHost;
use std::time::Duration;

#[derive(Clone, Data, Lens)]
struct TimeState {
    time: String,
}

#[derive(Clone, Data)]
struct TimeController;

impl<W: Widget<TimeState>> Controller<TimeState, W> for TimeController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut TimeState,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                let timer_id = ctx.request_timer(Duration::from_secs(1));
                ctx.set_handled();
            }
            Event::Timer(timer_id) => {
                data.time = data.get_time();
                ctx.request_paint();
                let timer_id = ctx.request_timer(Duration::from_secs(1));
                ctx.set_handled();
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

fn build_root_widget() -> impl Widget<TimeState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(24.0)
        .lens(TimeState::time)
        .padding(5.0);

    let display = ControllerHost::new(display, TimeController);

    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .cross_axis_alignment(CrossAxisAlignment::Start)
}

impl TimeState {
    fn get_time(&mut self) -> String {
        Utc::now().with_timezone(&New_York).to_string()
    }
}


pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("NewYork_Time: EST!")
        .window_size((249.0, 35.0))
        .resizable(false);

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
