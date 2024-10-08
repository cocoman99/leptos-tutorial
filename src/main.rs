use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (x, set_x) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_x.update(|n| *n += 10);
            }
            class:green=move || x.get() % 2 == 1
            // Set the `style` attribute
            style="position: absolute"
            style:left=move || format!("{}px", x.get() + 10)
            style:background-color=move || format!("rgb({}, {}, 100)", x.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", x)
        >
            "Click me: "
            {move || x.get()}
        </button>

        <progress
        max="50"
        // signals are functions, so `value=x` and `value=move || x.get()`
        // are interchangeable.
        value=move || x.get() * 2
        value=x
        />
    }
}