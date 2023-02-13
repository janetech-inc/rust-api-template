/*
 * Created January 22, 2022
 *
 * Copyright (c) 2023 - JaneTech Inc.
 * MIT License
 */
 
#[cfg(test)]
mod test_resource_route {

    use actix_web::{http::header::ContentType, test, web, App};
    use rust_docker_serverless::routes;

    #[actix_web::test]
    async fn test_get_resources_200() {
        let app = test::init_service(       App::new()
        .service(routes::resource::list)
  ).await;
        let req = test::TestRequest::get().uri("/objects").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_resource_200() {
        let app = test::init_service(     App::new()
        .service(routes::resource::get)).await;
        let req = test::TestRequest::get().uri("/objects/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

}

