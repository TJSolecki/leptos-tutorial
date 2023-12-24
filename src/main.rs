use ::leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let doubled_count = move || count() * 2;

    view! {
        <div style="display:flex; flex-direction:column; justify-content: center; align-items: center;">
            <button on:click=move |_| {
                set_count.update(|x| *x += 1)
            }>

                "The count is: " {count}
            </button>
            <ProgressBar max=50 progress=count/>
            <ProgressBar max=50 progress=doubled_count/>
        </div>
    }
}

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    #[prop(default = 100)] max: u16,
    progress: F,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // now this works
            value=progress
        ></progress>
    }
}
