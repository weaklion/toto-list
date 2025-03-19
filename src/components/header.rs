use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <nav class="h-12 font-bold text-xl w-full">
        {"TOTO List"}
      </nav>
    }
}
