/*
 * Created January 22, 2022
 *
 * Copyright (c) 2023 - JaneTech Inc.
 * MIT License
 */

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    pub id: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl Resource {
    pub fn _new() -> Self {
        Self {
            created_at: Some(Utc::now()),
            id: String::from(""),
        }
    }

    pub fn create(resource: &Resource) -> Resource {
        //TODO: impliment create

        return Resource {
            id: resource.id.to_owned(),
            created_at: resource.created_at,
        };
    }

    pub fn get(id: &str) -> Resource {
        //TODO: impliment get from source

        return Resource {
            id: id.to_owned(),
            created_at: Some(Utc::now()),
        };
    }

    pub fn update(id: &str, resource: &Resource) -> Resource {
        //TODO: impliment update

        return Resource {
            id: resource.id.to_owned(),
            created_at: resource.created_at,
        };
    }

    pub fn delete(id: &str) -> Resource {
        return Resource {
            id: id.to_owned(),
            created_at: Some(Utc::now()),
        };
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool {
       return self.id == other.id;
    }
}
