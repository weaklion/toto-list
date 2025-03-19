use gloo_timers::callback::Interval;
use yew::{
    function_component, html, use_effect_with, use_state, Callback, Html, MouseEvent, Properties,
};

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub title: String,
    pub completed: bool,
    pub id: usize,
    pub on_delete: Callback<usize>,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    let time_left = use_state(|| 60);
    let running = use_state(|| false);
    let open_menu = use_state(|| false);

    {
        let time_left = time_left.clone();
        let running = running.clone();

        use_effect_with(
            (running.clone(), time_left.clone()),
            move |(running, time_left)| {
                let interval = if **running && **time_left > 0 {
                    Some(Interval::new(1000, {
                        let time_left = time_left.clone();

                        move || {
                            if *time_left > 0 {
                                time_left.set(*time_left - 1);
                            }
                        }
                    }))
                } else {
                    None
                };

                move || {
                    if let Some(interval) = interval {
                        drop(interval);
                    }
                }
            },
        );
    }

    let start_timer = {
        let running = running.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            running.set(true);
        })
    };

    let reset_timer = {
        let running = running.clone();
        let time_left = time_left.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            running.set(false);
            time_left.set(60);
        })
    };

    let handle_delete = {
        let on_delete = props.on_delete.clone();
        let id = props.id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_delete.emit(id)
        })
    };

    let handle_menu = {
        let open_menu = open_menu.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            open_menu.set(!*open_menu);
        })
    };

    html! {
      <li class="flex items-center gap-x-2 my-2">
        <input class="form-check-input ml-2" type="checkbox" checked={props.completed} />
          {&props.title}

        if *running {
          <button class="px-2 py-1" onclick={reset_timer}>
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-pause"><rect x="14" y="4" width="4" height="16" rx="1"/><rect x="6" y="4" width="4" height="16" rx="1"/></svg>
          </button>
        } else {
          <button class="px-2 py-1" onclick={start_timer}>
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-play"><polygon points="6 3 20 12 6 21 6 3"/></svg>
          </button>
        }
        <button class="px-2 py-1" onclick={handle_menu}>
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-ellipsis-vertical"><circle cx="12" cy="12" r="1"/><circle cx="12" cy="5" r="1"/><circle cx="12" cy="19" r="1"/></svg>
        </button>

        {format!("{}초 남음", *time_left)}

        <button class="border rounded px-2 py-1" onclick={handle_delete} >{"삭제"}</button>
      </li>
    }
}
