cfg_if::cfg_if! {

    if #[cfg(feature = "ssr")] {

        use crate::app::models::Person;
        use crate::app::errors::{ ErrorMessage, PersonError };
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() {

            let _ = DB.connect::<Ws>("127.0.0.1:8000").await;
            let _ = DB.signin(Root {
                username: "Root",
                password: "Root",
            })
            .await;
            let _ = DB.use_ns("surreal").use_db("person").await;
        }

        pub async fn get_all_persons() -> Option<Vec<Person>> {

            open_db_connection().await;
            let get_all_persons = DB.query("SELECT * FROM person ORDER BY joined_date DESC;").await;
            let _ = DB.invalidate().await;

            match get_all_persons {

                Ok(mut res) => {
                    let found = res.take(0);
                    match found {
                        Ok(found_persons) => Some(found_persons),
                        Err(_) => None,
                    }
                },
                Err(_) => None,
            }
        }

        pub async fn add_person(new_person: Person) -> Option<Person> {

            open_db_connection().await;
            let results = DB.create(("person", new_person.uuid.clone()))
                .content(new_person)
                .await;
            let _ = DB.invalidate().await;

            match results {
                Ok(created_person) => created_person,
                Err(e) => {
                    println!("error in adding person: {:?}",e);
                    None
                }
            }
        }

        pub async fn delete_person(person_uuid: String)
            -> Result<Option<Person>, PersonError> {

            open_db_connection().await;
            let delete_results = DB.delete(("person",person_uuid)).await;
            let _ = DB.invalidate().await;

            match delete_results {
                Ok(deleted_person) => Ok(deleted_person),
                Err(_) => Err(PersonError::PersonDeleteFailure)
            }
        }

        pub async fn update_person(uuid: String, title: String, level: String,
            compensation: i32) -> Result<Option<Person>,PersonError> {

            open_db_connection().await;

            // first we try to find the perosn in the database
            let find_person: Result<Option<Person>,Error> =
                DB.select(("person",&uuid)).await;
            match find_person {

                Ok(found) => {

                    // if we found the person, we update him/her
                    match found {
                        Some(found_person) => {

                            let updated_user: Result<Option<Person>,Error> =
                                DB.update(("person",&uuid))
                                .merge(Person::new(
                                    uuid,
                                    found_person.name,
                                    title,
                                    level,
                                    compensation,
                                    found_person.joined_date
                                ))
                                .await;
                            let _ = DB.invalidate().await;
                            match updated_user {
                                Ok(returned_user) => Ok(returned_user),
                                Err(_) => Err(PersonError::PersonUpdateFailure)
                            }
                        },
                        None => Err(PersonError::PersonUpdateFailure)
                    }
                },
                Err(_) => {
                    let _ = DB.invalidate().await;
                    Err(PersonError::PersonNotFound)
                }
            }
        }
    }
}
