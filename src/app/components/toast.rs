use leptos::*;
use std::time::Duration;

const TOAST_PARENT_STYLE: &str = "flex flex-row top-0 h-16 w-full max-w-[61rem]
    mx-auto items-center justify-center align-center fixed -mt-36
    transition-all duration-1000 ease-in-out";

const TOAST_PARENT_APPEAR_STYLE: &str = "flex flex-row top-0 h-16 w-full max-w-[61rem]
    mx-auto items-center justify-center align-center fixed mt-20
    transition-all duration-1000 ease-in-out";

const TOAST_STYLE: &str = "flex w-96 h-16 bg-[#333333] rounded px-10 py-4
    text-white -mt-36 transition-all duration-1000 ease-in-out items-center";

pub enum ToastMessageType {
    NewMemberAdded,
    MemberDeleted,
    MemberUpdated,
}

pub type ToastMessage = String;

pub trait Toast {
    fn create(toast_message_type: ToastMessageType) -> ToastMessage;
}

impl Toast for ToastMessage {
    fn create(toast_message_type: ToastMessageType) -> ToastMessage {
        match toast_message_type {
            ToastMessageType::NewMemberAdded => String::from("New member added"),
            ToastMessageType::MemberDeleted => String::from("Member deleted"),
            ToastMessageType::MemberUpdated => String::from("Member updated"),
        }
    }
}

#[component]
pub fn Toast(
    toast_message: ReadSignal<ToastMessage>,
    if_appear: ReadSignal<bool>,
    set_if_appear: WriteSignal<bool>,
) -> impl IntoView {
    let hide = move || {
        set_if_appear(false);
    };

    create_effect(move |_| {
        if if_appear() {
            set_timeout(hide, Duration::from_secs(4));
        }
    });

    view! {
        <div class={move || {
            if if_appear() { TOAST_PARENT_APPEAR_STYLE }
            else { TOAST_PARENT_STYLE }
        }}>
            <div class=TOAST_STYLE>
                {toast_message}
            </div>
        </div>
    }
}
