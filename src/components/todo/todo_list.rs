use yew::{function_component, html, Html, Properties};

use super::types::Todo;
use crate::components::todo::todo_item::TodoItem;

#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    pub todo_items: Vec<Todo>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    html!(
      <ul>
        {props.todo_items.iter().map(|todo| html! {
          <TodoItem title={todo.title.clone()} completed={todo.completed} />
        }).collect::<Html>()}
      </ul>
    )
}
