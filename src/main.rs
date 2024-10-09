use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {

    view! {
        <h1 style="background-color:#a6edc1;">GUESS-THE-NUMBER</h1>
        
    }
}

