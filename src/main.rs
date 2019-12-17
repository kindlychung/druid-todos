use std::sync::Arc;

use druid::lens::LensWrap;
use druid::piet::Color;
use druid::widget::{
    Button, Checkbox, DynLabel, EnvScope, Flex, Label, List, Scroll, TextBox, WidgetExt,
};
use druid::{theme, AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc};

const LAYOUT_BASE: f64 = 8.0;
const TOOLBAR_HEIGHT: f64 = LAYOUT_BASE * 6.;

fn toolbar_color() -> Color {
    Color::rgb(0.20, 0.20, 0.20)
}

fn set_header_footer_env(env: &mut Env) {
    env.set(theme::BUTTON_DARK, Color::rgb(0.30, 0.30, 0.30));
    env.set(theme::BUTTON_LIGHT, Color::rgb(0.35, 0.35, 0.35));
    env.set(theme::BACKGROUND_LIGHT, Color::rgb(0.3, 0.3, 0.3));
}

#[derive(Clone, Data, Lens)]
struct TodoItem {
    task: String,
    is_completed: bool,
}

impl TodoItem {
    fn new(task: &str, is_completed: bool) -> Self {
        TodoItem {
            task: String::from(task),
            is_completed,
        }
    }
}

#[derive(Clone, Data, Lens)]
struct AppState {
    todos: Arc<Vec<TodoItem>>,
    current_entry: String,
}

impl AppState {
    fn default() -> Self {
        AppState {
            todos: Arc::new(vec![
                TodoItem::new("Build app", false),
                TodoItem::new("Save the world", true),
                TodoItem::new("Play video games", false),
            ]),
            current_entry: "".to_string(),
        }
    }
}

fn main() {
    let window_title = LocalizedString::new("Druid Todos");
    let main_window = WindowDesc::new(ui_builder).title(window_title);
    let data = get_initial_state();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<AppState> {
    let add_todo_label = Label::new(LocalizedString::new("Add todo:")).padding(LAYOUT_BASE);
    let add_todo_textbox = TextBox::new()
        .lens(AppState::current_entry)
        .padding(LAYOUT_BASE);

    let add_todo_button = Button::new(
        LocalizedString::new("Add"),
        |_, app_state: &mut AppState, _| {
            Arc::make_mut(&mut app_state.todos)
                .push(TodoItem::new(&app_state.current_entry.clone(), false));
            app_state.current_entry = "".to_string();
        },
    )
    .padding(LAYOUT_BASE)
    .fix_width(LAYOUT_BASE * 10.);

    let header = EnvScope::new(
        set_header_footer_env,
        Flex::row()
            .with_child(add_todo_label, 0.0)
            .with_child(add_todo_textbox, 1.0)
            .with_child(add_todo_button, 0.0)
            .fix_height(TOOLBAR_HEIGHT)
            .background(toolbar_color()),
    );

    // Is there a way to add a border between this list and the textbox above?
    let todo_list = Scroll::new(List::new(|| {
        let checkbox = LensWrap::new(Checkbox::new(), TodoItem::is_completed).padding(LAYOUT_BASE);

        // How does this DynLabel have access to the individual TodoItem?
        let label = DynLabel::new(|todo_item: &TodoItem, _| format!("{}", todo_item.task))
            .padding(LAYOUT_BASE);

        Flex::row().with_child(checkbox, 0.0).with_child(label, 0.0)
    }))
    .padding(LAYOUT_BASE)
    .lens(AppState::todos);

    let clear_completed_button = Button::new(
        LocalizedString::new("Clear Completed"),
        |_, todos: &mut Arc<Vec<TodoItem>>, _| {
            Arc::make_mut(todos).retain(|item| !item.is_completed);
        },
    )
    .padding(LAYOUT_BASE)
    .fix_width(LAYOUT_BASE * 18.)
    .lens(AppState::todos);

    let status_label = DynLabel::new(|app_state: &AppState, _| {
        format!(
            "{} todos, {} incomplete",
            app_state.todos.len(),
            app_state
                .todos
                .iter()
                .filter(|&todo| !todo.is_completed)
                .count()
        )
    })
    .padding(LAYOUT_BASE);

    let footer = EnvScope::new(
        set_header_footer_env,
        Flex::row()
            .with_child(status_label, 1.0)
            .with_child(clear_completed_button, 0.)
            .fix_height(TOOLBAR_HEIGHT)
            .background(toolbar_color()),
    );

    Flex::column()
        .with_child(header, 0.)
        .with_child(todo_list, 1.)
        .with_child(footer, 0.)
}

fn get_initial_state() -> AppState {
    AppState::default()
}
