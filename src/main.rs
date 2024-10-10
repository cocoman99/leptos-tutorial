use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let mut user_number: u32 = 4;

    view! {
        <h1 style="background-color:#a6edc1;">* * * GUESS-THE-NUMBER * * *</h1>
            <div class="main">        
                <div>"CHOOSE NUMBER FROM 1 TO 100."</div>
                <div>
                "INPUT YOUR GUESS "<button
                    on:click=move |_| {
                        // on stable, this is set_count.set(3);
                        set_count.set(user_number);
                    }
                    >
                    "HERE"
                </button>                
                </div>
                "THEN CLICK "
                <button
                    on:click=move |_| {
                        // on stable, this is set_count.set(3);
                        //count(user_number);
                    }
                    >
                    "HERE"
                </button>" TO CHECK"
            </div>
    }
}