/*
 * Created January 22, 2022
 *
 * Copyright (c) 2023 - JaneTech Inc.
 * MIT License
 */


use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::{delete, get, post, put, HttpResponse};

use crate::constants::APPLICATION_JSON;
use crate::models::resource::Resource;
use crate::models::response::Response;

pub type Resources = Response<Resource>;

#[get("/objects")]
pub async fn list() -> HttpResponse {
    let resources = Resources { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(resources)
}

#[post("/objects")]
pub async fn create(resource: Json<Resource>) -> HttpResponse {
    
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Resource::create(&resource.into_inner()))
}

#[get("/objects/{id}")]
pub async fn get(id: String) -> HttpResponse {
    
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Resource::get(&id))
}

#[put("/objects/{id}")]
pub async fn update(id: String, resource: Json<Resource>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Resource::update(&id, &resource.into_inner()))
}

#[delete("/objects/{id}")]
pub async fn delete(id: String) -> HttpResponse {

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Resource::delete(&id))
}
