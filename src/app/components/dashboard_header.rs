use chrono::{Datelike, Local, Month};
use leptos::*;

pub fn DashboardHeader() -> impl IntoView {
    // get today's date
    let current_now = Local::now();

    // get the current month's number
    let month_number = u8::try_from(current_now.month()).unwrap();

    // then we get the current month in shortform (e.g Jan/Feb)
    let current_month = Month::try_from(month_number).ok().unwrap();

    // then we display the month and year in a nice format
    let display_date = format!("{:?}, {:?}", current_month, current_now.year());

    view! {

        <div class="flex flex-col mt-28 h-48 w-full max-w-[53rem] mx-auto
            items-center align-center justify-center px-2">
            <div class="w-full flex flex-row bg-[#34508c] rounded h-full px-10
                py-10">
                <div class="w-1/2 h-full flex flex-col">
                    <div class="text-white">{display_date}</div>
                    <div class="text-white text-6xl pt-2">"Team Report"</div>
                </div>
                <div class="w-1/2">
                    <img src="assets/image_1.png" class="w-[442px] -mt-28" />
                </div>
            </div>
        </div>
    }
}
