use crate::app::components::{Toast, ToastMessage, ToastMessageType};
use crate::app::models::{EditPersonRequest, Person};
use crate::app::server_functions::persons::edit_person;
use leptos::*;
use std::rc::Rc;
use validator::Validate;

// style for each field, including subtle animations for animating the
// placeholder

// when focused upon
const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-4 pl-6 py-4 text-white
    mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000
    ease-in-out";

const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded
    text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";

const UPDATE_BUTTON_STYLE: &str = "mt-10 bg-[#7734e7] px-8 py-2 rounded
    text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8
    border-[#7734e7] px-6 pt-5 h-[29rem] w-full max-w-[36rem] z-50 -mt-2 fixed
    top-20 z-50";

const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8
    border-[#7734e7] px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed
    top-20 z-50";

#[component]
pub fn EditPersonModal(
    person: Rc<Person>,
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_toast: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
    person_resource: Resource<(), Result<Vec<Person>, ServerFnError>>,
) -> impl IntoView {
    let (person_name, _set_person_name) = create_signal(person.name.clone());
    let (person_title, set_person_title) = create_signal(person.title.clone());
    let (person_level, set_person_level) = create_signal(person.level.clone());
    let (compensation, set_compensation) = create_signal(format!("{}", person.compensation));

    // for error messages
    let (error_message, set_error_message) = create_signal(String::new());
    let (if_error, set_if_error) = create_signal(false);

    // to close the modal
    let on_close = move |_| {
        set_if_show_modal(false);
    };

    // to perform the updating of the person
    let on_click = move |_| {
        let uuid = person.uuid.clone();

        // we first validate if the compensation is a valid number
        let validated_compensation = compensation().parse::<i32>();

        // if no issues with validating the compensation,
        if let Ok(ok_compensation) = validated_compensation {
            let edit_person_request =
                EditPersonRequest::new(uuid, person_title(), person_level(), ok_compensation);

            let is_valid = edit_person_request.validate();

            match is_valid {
                Ok(_) => {
                    let _ = spawn_local(async move {
                        // we call the server function here
                        let edit_result = edit_person(edit_person_request).await;

                        // we get the result back and do something with it
                        match edit_result {
                            Ok(_edited_person) => {
                                person_resource.refetch();

                                set_if_show_modal(false);

                                set_toast_message(ToastMessage::create(
                                    ToastMessageType::MemberUpdated,
                                ));

                                // setting this to true to make the toast appear
                                set_if_show_toast(true);
                            }
                            Err(_e) => {
                                set_if_error(true);
                                set_error_message(String::from(
                                    "Error updating member. Please try again later",
                                ))
                            }
                        };
                    });
                }
                Err(_e) => {
                    set_if_error(true);
                    set_error_message(String::from("All fields are required"))
                }
            }
        } else {
            set_if_error(true);
            set_error_message(String::from("Compensation should be numeric"))
        }
    };

    view! {

        <div class="flex flex-col w-full h-full z-50 mx-auto items-center
            align-center">

            <div class={ move || {
                if if_error() { ERROR_STYLE }
                else { NO_ERROR_STYLE }
            }}>

                <Show when=move || { if_error() }>
                    <p class="text-white bg-red-500 rounded w-full h-12 px-5
                        py-3 transition-all duration-750 ease-in-out">
                        { error_message() }
                    </p>
                </Show>
                <p class="text-white pt-5 text-4xl mb-10">{person_name}</p>

                <input type="text" placeholder="Title" class=INPUT_STYLE
                    value=person_title
                    on:input=move |event| {
                        set_person_title(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Level" class=INPUT_STYLE
                    value=person_level
                    on:input=move |event| {
                        set_person_level(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Compensation" class=INPUT_STYLE
                    value=compensation
                    on:input=move |event| {
                        set_compensation(event_target_value(&event));
                    }
                />

                <div class="flex flex-row w-full items-right justify-right mt-3">

                    <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                        "Cancel"
                    </button>
                    <button on:click=on_click class=UPDATE_BUTTON_STYLE>
                        "Update"
                    </button>
                </div>
            </div>
        </div>
    }
}
