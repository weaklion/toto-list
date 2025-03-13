use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <nav class="bg-slate-50 h-8 w-full">
        {"Yew Todo App"}
      </nav>
    }
}
