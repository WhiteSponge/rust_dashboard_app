use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Person {
    pub uuid: String,
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "level is required"))]
    pub level: String,
    #[validate(range(min = 2000, max = 99999))]
    pub compensation: i32,
    pub joined_date: String,
}

impl Person {
    pub fn new(
        uuid: String,
        name: String,
        title: String,
        level: String,
        compensation: i32,
        joined_date: String,
    ) -> Person {
        Person {
            uuid,
            name,
            title,
            level,
            compensation,
            joined_date,
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct AddPersonRequest {
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "level is required"))]
    pub level: String,
    #[validate(range(min = 2000, max = 99999))]
    pub compensation: i32,
}

impl AddPersonRequest {
    pub fn new(name: String, title: String, level: String, compensation: i32) -> AddPersonRequest {
        AddPersonRequest {
            name,
            title,
            level,
            compensation,
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct EditPersonRequest {
    #[validate(length(min = 1, message = "id is required"))]
    pub uuid: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "level is required"))]
    pub level: String,
    #[validate(range(min = 2000, max = 99999))]
    pub compensation: i32,
}

impl EditPersonRequest {
    pub fn new(uuid: String, title: String, level: String, compensation: i32) -> EditPersonRequest {
        EditPersonRequest {
            uuid,
            title,
            level,
            compensation,
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct DeletePersonRequest {
    #[validate(length(min = 1, message = "id is required"))]
    pub uuid: String,
}

impl DeletePersonRequest {
    pub fn new(uuid: String) -> DeletePersonRequest {
        DeletePersonRequest { uuid }
    }
}
