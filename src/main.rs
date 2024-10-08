use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(1);

    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                //set_count(3);
                set_count.update(|n| *n *= 2);
            }
        >
            "CLICK HERE: "
            // on stable, this is move || count.get();
            {move || count.get()}
        </button>
    }
}