use yew::{function_component, html, Callback, Html, Properties};

use super::types::Todo;
use crate::components::todo::todo_item::TodoItem;

#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    pub todo_items: Vec<Todo>,
    pub on_delete: Callback<usize>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    html!(
      <ul>
        {props.todo_items.iter().map(|todo| html! {
          <TodoItem title={todo.title.clone()} completed={todo.completed} id={todo.id} on_delete={props.on_delete.clone()}/>
        }).collect::<Html>()}
      </ul>
    )
}
