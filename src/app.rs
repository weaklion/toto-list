use std::rc::Rc;

use crate::components::{
    header::Header,
    todo::{todo_form::TodoForm, todo_list::TodoList, types::Todo},
};

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let todo_items = use_state(|| {
        vec![
            Todo {
                id: 1,
                title: "Learn Rust".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Learn Yew".to_string(),
                completed: true,
            },
        ]
    });
    let next_id = use_state(|| 1);

    let on_add = {
        let todo_items = todo_items.clone();
        Callback::from(move |title: String| {
            let mut current_todo_items = (*todo_items).clone();
            current_todo_items.push(Todo {
                id: *next_id,
                title,
                completed: false,
            });
            next_id.set(*next_id + 1);
            todo_items.set(current_todo_items);
        })
    };

    let on_delete = {
        let todo_items = todo_items.clone();
        Callback::from(move |id: usize| {
            let current_todo_items = (*todo_items).clone();
            let updated_todo_items = current_todo_items
                .into_iter()
                .filter(|todo| todo.id != id)
                .collect::<Vec<_>>();
            todo_items.set(updated_todo_items);
        })
    };

    html! {
        <>
        <Header/>
        <main class="container">
        <div class="mt-5">
            <TodoForm {on_add} />
            <TodoList todo_items={(*todo_items).clone()} {on_delete}  />
        </div>
        </main>
        </>
    }
}
