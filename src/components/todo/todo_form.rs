use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, html, use_state, Callback, Html, MouseEvent, Properties,
    TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct TodoFormProps {
    pub on_add: Callback<String>,
}

#[function_component(TodoForm)]
pub fn todo_form(props: &TodoFormProps) -> Html {
    let title = use_state(|| "".to_string());

    let onchange = {
        let title = title.clone();
        Callback::from(move |e: Event| {
            // 한글 지원을 위해서 htmlInputElement를 쓴다. data()로 가져오면 한글 지원이 안됨.
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            title.set(value);
        })
    };

    let onclick = {
        let on_add = props.on_add.clone();
        let title = title.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_add.emit((*title).clone());
            title.set("".to_string())
        })
    };

    html! {
      <form class="mb-5">
      <div class="flex items-center gap-x-2 ">
          <input type="text" class="border rounded" {onchange} value={(*title).clone()} id="title" />
        <button type="submit" class="border rounded-lg px-4 py-2" {onclick}>{"추가"}</button>
        </div>
      </form>
    }
}
