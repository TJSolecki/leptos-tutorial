use ::leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
#[component]
fn App() -> impl IntoView {
    let static_list_of_counters = vec![create_signal(0), create_signal(1), create_signal(2)];
    view! {
        <div style="display:flex; flex-direction:column; justify-content: center; align-items: center;">
            <ul>

                {static_list_of_counters
                    .into_iter()
                    .map(|(count, set_count)| {
                        return view! {
                            <li>
                                <button on:click=move |_| {
                                    set_count.update(|x| *x += 1)
                                }>{count}</button>
                            </li>
                        }
                    })
                    .collect_view()}

            </ul>
            <DynamicCounterList/>
        </div>
    }
}

#[component]
fn DynamicCounterList() -> impl IntoView {
    let initial_counters: Vec<_> = vec![(0, create_signal(0))];
    let mut num_counters = 1;
    let (counters, set_counters) = create_signal(initial_counters);
    view! {
        <button on:click=move |_| {
            let sig = create_signal(num_counters + 1);
            set_counters.update(move |counters| { counters.push((num_counters, sig)) });
            num_counters += 1;
        }>"Add counter"</button>
        <ul>
            <For
                // the key should be unique and stable for each row
                // using an index is usually a bad idea, unless your list
                // can only grow, because moving items around inside the list
                // means their indices will change and they will all rerender
                each=counters
                key=|counter| counter.0
                children=move |(key, (count, set_count))| {
                    view! {
                        <li>
                            <button on:click=move |_| set_count.update(|x| *x += 1)>{count}</button>
                            <button on:click=move |_| {
                                set_counters
                                    .update(|counters| {
                                        counters.retain(|(counter_id, _)| counter_id != &key)
                                    });
                            }>"Remove"</button>
                        </li>
                    }
                }
            />

        </ul>
    }
}
