use druid::widget::{Flex, Label};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};

fn main() {
    let window_title = LocalizedString::new("Druid Todos");
    let main_window = WindowDesc::new(todos_builder).title(window_title);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn todos_builder() -> impl Widget<u32> {
    let text = LocalizedString::new("Todos");
    let label = Label::new(text);

    Flex::column().with_child(label, 1.0)
}
