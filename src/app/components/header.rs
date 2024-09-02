use leptos::*;
use leptos_router::*;

const INPUT_STYLE: &str = "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4
    hover:border-b-2";
const INPUT_STYLE_SELECTED: &str = "border-b-2 border-[#9734e7] h-8 text-white ml-4
    mr-4 hover:border-b-2";

#[component]
pub fn Header() -> impl IntoView {
    let (current_path, set_current_path) = create_signal(String::new());

    create_effect(move |_| {
        // here we get the current url and save it
        let router_context = use_context::<RouterContext>();
        match router_context {
            Some(route_context) => {
                let path = route_context.pathname().get();
                set_current_path(path);
            }
            None => {
                set_current_path(String::from("/"));
            }
        }
    });

    view! {

        <div class="flex mx-auto align-center items-center w-full h-12 pt-8 px-20 top-0 fixed">
            <nav class="flex flex-row w-full max-w-[52rem] h-12">
                <div class={move || get_style_from_url(&current_path, "/")}>
                    <A href="/">"Dashboard"</A>
                </div>
                <div class={move || get_style_from_url(&current_path, "/team")}>
                    <A href="/team">"Team"</A>
                </div>
            </nav>
        </div>
    }
}

fn get_style_from_url<'a, 'b>(url: &'a ReadSignal<String>, match_url: &'a str) -> &'b str {
    if url() == match_url {
        return INPUT_STYLE_SELECTED;
    }

    INPUT_STYLE
}
