use crate::app::{
    errors::{ErrorMessage, ResponseErrorTrait},
    models::{person::Person, AddPersonRequest, DeletePersonRequest, EditPersonRequest},
};
use leptos::*;
use serde::*;

#[server(GetPersons, "/api")]
pub async fn get_persons() -> Result<Vec<Person>, ServerFnError> {
    let persons = retrieve_all_persons().await;
    Ok(persons)
}

#[server(AddPerson, "/api")]
pub async fn add_person(add_person_request: AddPersonRequest) -> Result<Person, ServerFnError> {
    let new_person = add_new_person(
        add_person_request.name,
        add_person_request.title,
        add_person_request.level,
        add_person_request.compensation,
    )
    .await;

    match new_person {
        Some(created_person) => Ok(created_person),
        None => Err(ServerFnError::Args(String::from(
            "Error in creating person!",
        ))),
    }
}

#[server(DeletePerson, "/api")]
pub async fn delete_person(
    delete_person_request: DeletePersonRequest,
) -> Result<Person, ServerFnError> {
    let deleted_results = delete_team_person(delete_person_request.uuid).await;
    match deleted_results {
        Ok(deleted) => {
            if let Some(deleted_person) = deleted {
                Ok(deleted_person)
            } else {
                Err(ServerFnError::Response(ErrorMessage::create(
                    PersonError::PersonDeleteFailure,
                )))
            }
        }
        Err(person_error) => Err(ServerFnError::Response(ErrorMessage::create(person_error))),
    }
}

#[server(EditPerson, "/api")]
pub async fn edit_person(edit_person_request: EditPersonRequest) -> Result<Person, ServerFnError> {
    let updated = edit_team_person(
        edit_person_request.uuid,
        edit_person_request.title,
        edit_person_request.level,
        edit_person_request.compensation,
    )
    .await;

    match updated {
        Ok(updated_result) => {
            // if successfully returned a Some in an Option of a Person
            if let Some(updated_person) = updated_result {
                Ok(updated_person)
            } else {
                Err(ServerFnError::Args(ErrorMessage::create(
                    PersonError::PersonUpdateFailure,
                )))
            }
        }
        Err(person_error) => Err(ServerFnError::Args(ErrorMessage::create(person_error))),
    }
}

cfg_if::cfg_if! {

    if #[cfg(feature = "ssr")] {

        use crate::app::db::database;
        use crate::app::errors::{ PersonError };
        use chrono::{DateTime, Local};
        use uuid::Uuid;

        pub async fn retrieve_all_persons() -> Vec<Person> {

            let get_all_persons_result = database::get_all_persons().await;
            match get_all_persons_result {
                Some(found_persons) => found_persons,
                None => Vec::new()
            }
        }

        pub async fn add_new_person<T>(name: T, title: T, level: T, compensation:i32)
            -> Option<Person> where T: Into<String> {

            let mut buffer = Uuid::encode_buffer();
            let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            // getting the current timestamp
            let current_now = Local::now();
            let current_formatted = current_now.to_string();

            let new_person = Person::new(
                String::from(uuid),
                name.into(),
                title.into(),
                level.into(),
                compensation,
                current_formatted
            );

            database::add_person(new_person).await
        }

        pub async fn delete_team_person<T>(uuid: T) ->
            Result<Option<Person>,PersonError>
            where T: Into<String> {

            database::delete_person(uuid.into()).await
        }

        pub async fn edit_team_person<T>(uuid: T, title: T, level: T,
            compensation:i32) -> Result<Option<Person>,PersonError>
            where T:Into<String> {

            database::update_person(uuid.into(),title.into(),level.into(),
                compensation).await
        }
    }
}
