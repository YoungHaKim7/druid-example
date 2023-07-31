use druid::{
    widget::{Flex, Label},
    AppLauncher, PlatformError, Widget, WindowDesc, WidgetExt
};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let data = "00:00:00".into();
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<String> {
    // The label text will be computed dynamically based on the current locale and count
    let text = "new".to_string();
    let label = Label::new(text).padding(5.0).center();

    Flex::column().with_child(label)
}
