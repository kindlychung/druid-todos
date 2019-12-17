use std::sync::Arc;

use druid::lens::LensWrap;
use druid::widget::{Button, Checkbox, DynLabel, Flex, Label, List, Scroll, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};

const PADDING_BASE: f64 = 8.0;

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
    let main_window = WindowDesc::new(todos_builder).title(window_title);
    let data = get_initial_state();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn todos_builder() -> impl Widget<AppState> {
    // How do I vertically align this label in the center?
    let add_todo_label = Label::new(LocalizedString::new("Add todo:")).padding(PADDING_BASE);
    let add_todo_textbox = TextBox::new()
        .lens(AppState::current_entry)
        .padding(PADDING_BASE);
    
    // How do I make this button expand to fill the height of the flex box?
    // Also, is there such a thing as inner padding for buttons?
    let add_todo_button = Button::new(
        LocalizedString::new("Add"),
        |_, app_state: &mut AppState, _| {
            Arc::make_mut(&mut app_state.todos)
                .push(TodoItem::new(&app_state.current_entry.clone(), false));
            app_state.current_entry = "".to_string();
        },
    )
    .padding(PADDING_BASE);

    let clear_completed_button = Button::new(
        LocalizedString::new("Clear Completed"),
        |_, todos: &mut Arc<Vec<TodoItem>>, _| {
            Arc::make_mut(todos).retain(|item| !item.is_completed);
        },
    )
    .padding(PADDING_BASE)
    .lens(AppState::todos);

    let new_todo_row = Flex::row()
        .with_child(add_todo_label, 0.0)
        .with_child(add_todo_textbox, 1.0)
        .with_child(add_todo_button, 0.0)
        .with_child(clear_completed_button, 0.0);

    // Is there a way to add a border between this list and the textbox above?
    let todo_list = Scroll::new(List::new(|| {
        let checkbox = LensWrap::new(Checkbox::new(), TodoItem::is_completed).padding(PADDING_BASE);

        // How does this DynLabel have access to the individual TodoItem?
        let label = DynLabel::new(|todo_item: &TodoItem, _| format!("{}", todo_item.task))
            .padding(PADDING_BASE);

        Flex::row().with_child(checkbox, 0.0).with_child(label, 0.0)
    }))
    .lens(AppState::todos);

    Flex::column()
        .with_child(new_todo_row, 0.0)
        .with_child(todo_list, 1.0)
}

fn get_initial_state() -> AppState {
    AppState::default()
}
