use crate::app::components::{EditPersonModal, ShowPersonModal, ToastMessage};
use crate::app::models::Person;
use leptos::*;
use std::rc::Rc;

const ROW_STYLE: &str = "bg-[#283653] rounded px-10 py-5 mb-4 flex flex-row
    text-left items-left transition-all duration-1000 ease-in-out";

const SHOW_ICON_STYLE: &str = "bg-transparent border-2 border-white px-2.5 mt-2
    rounded-full text-white transition-all duration-500 ease-in-out text-xs
    mr-3 w-7 h-7 hover:w-8 hover:h-8 hover:mb-1";

#[component]
pub fn PersonRow(
    person: Rc<Person>,
    person_resource: Resource<(), Result<Vec<Person>, ServerFnError>>,
    set_if_show_toast: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
) -> impl IntoView {
    let (if_show_info_modal, set_if_show_info_modal) = create_signal(false);
    let (if_show_edit_modal, set_if_show_edit_modal) = create_signal(false);

    let on_show_info = move |_| {
        set_if_show_info_modal(true);
    };

    let on_show_edit = move |_| {
        set_if_show_edit_modal(true);
    };

    let edit_person = person.clone();
    let team_person = person.clone();

    view! {

        <Show when=move || { if_show_info_modal() }>
            <ShowPersonModal
                person=team_person.clone()
                set_if_show_modal=set_if_show_info_modal
                set_if_show_deleted=set_if_show_toast
                person_resource
                set_toast_message
            />
        </Show>

        <Show when=move || { if_show_edit_modal() }>
            <EditPersonModal
                person=edit_person.clone()
                set_if_show_modal=set_if_show_edit_modal
                set_if_show_toast=set_if_show_toast
                person_resource
                set_toast_message
            />
        </Show>
        <div class=ROW_STYLE>
            <div class="flex flex-col w-full max-w-[45rem]">
                <p class="font-bold">{&person.name}</p>
                <p class="text-sm text-stone-400">{&person.title}</p>
            </div>

            <div class="flex flex-row">
                <button class=SHOW_ICON_STYLE on:click=on_show_info>"i"</button>
                <button class="" on:click=on_show_edit>
                    <img src="assets/edit.png" class="w-[35px] hover:w-[38px]
                        transition-all duration-500" />
                </button>
            </div>
        </div>
    }
}
