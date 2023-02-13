/*
 * Created January 22, 2022
 *
 * Copyright (c) 2023 - JaneTech Inc.
 * MIT License
 */


use actix_web::web::Path;
use actix_web::{get, HttpResponse};

use crate::constants::APPLICATION_JSON;

#[get("/")]
pub async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(vec![""])
}
