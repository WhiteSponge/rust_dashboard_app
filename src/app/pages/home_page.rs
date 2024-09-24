use crate::app::components::{DashboardChart, DashboardHeader, Header};
use crate::app::server_functions::get_persons;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let get_persons_info = create_resource(|| (), |_| async move { get_persons().await });

    view! {
        <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
            <Header />
            <DashboardHeader />
            <Suspense fallback=move || {
                view! { <p>"Loading data..."</p> }
            }>
                {
                    move || {
                        get_persons_info.get().map(|data| {
                            match data {
                                Ok(persons_data) => {
                                    view! {
                                        <DashboardChart persons_data />
                                    }.into_view()
                                },
                                Err(_) => view! {
                                    <div></div>
                                }.into_view()
                            }
                        })
                    }
                }
            </Suspense>
        </div>
    }
}
