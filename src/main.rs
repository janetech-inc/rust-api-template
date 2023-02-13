/*
 * Created January 22, 2022
 *
 * Copyright (c) 2023 - JaneTech Inc.
 * MIT License
 */
 
use lambda_web::actix_web::{self, get, middleware, App, HttpServer, Responder};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};

use std::{env, io};

mod constants;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let factory = move || {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(routes::home::home)
            .service(routes::resource::list)
            .service(routes::resource::create)
            .service(routes::resource::get)
    };
    ();

    if running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Run local server
        HttpServer::new(factory).bind("0.0.0.0:8080")?.run().await?;
    }
    Ok(())
}

pub fn create_app() {
    App::new()
        // enable logger - always register actix-web Logger middleware last
        .wrap(middleware::Logger::default())
        // register HTTP requests handlers
        .service(routes::home::home)
        .service(routes::resource::list)
        .service(routes::resource::create)
        .service(routes::resource::get);
}

/// Returns true if it is running on AWS Lambda
pub fn running_on_lambda() -> bool {
    std::env::var("AWS_LAMBDA").is_ok() || is_running_on_lambda()
}
