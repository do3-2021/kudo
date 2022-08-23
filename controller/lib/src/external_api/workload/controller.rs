use super::{model::Pagination, model::WorkloadDTO, model::WorkloadError, service};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, Scope};

struct WorkloadController {}

impl WorkloadController {
    pub async fn get_workload(params: web::Path<(String, String)>) -> impl Responder {
        let (namespace, workload_id) = params.into_inner();

        let mut workload_service = service::WorkloadService::new().await;
        match workload_service
            .get_workload(&workload_id, &namespace)
            .await
        {
            Ok(workload) => HttpResponse::build(StatusCode::OK).body(workload),
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => {
                    HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found")
                }
                _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal Server Error"),
            },
        }
    }
    pub async fn put_workload(
        namespace: web::Path<String>,
        body: web::Json<WorkloadDTO>,
    ) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;

        let workload_dto = body.into_inner();
        match workload_service
            .create_workload(workload_dto, &namespace)
            .await
        {
            Ok(workload) => HttpResponse::build(StatusCode::CREATED).body(workload),
            Err(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body("Internal Server Error")
            }
        }
    }

    pub async fn get_all_workloads(
        namespace: web::Path<String>,
        pagination: Option<web::Query<Pagination>>,
    ) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;

        match pagination {
            Option::Some(pagination) => {
                let workloads = workload_service
                    .get_all_workloads(pagination.limit, pagination.offset, &namespace)
                    .await;
                return web::Json(workloads);
            }
            Option::None => {
                let workloads = workload_service.get_all_workloads(0, 0, &namespace).await;
                return web::Json(workloads);
            }
        }
    }
    pub async fn patch_workload(
        params: web::Path<(String, String)>,
        body: web::Json<WorkloadDTO>,
    ) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;

        let (workload_id, namespace) = params.into_inner();
        let workload_dto = body.into_inner();

        match workload_service
            .update_workload(workload_dto, &workload_id, &namespace)
            .await
        {
            Ok(workload) => HttpResponse::build(StatusCode::CREATED).body(workload),
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => {
                    HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found")
                }
                _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal Server Error"),
            },
        }
    }

    pub async fn delete_workload(
        namespace: web::Path<String>,
        workload_id: web::Path<String>,
    ) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;

        match workload_service
            .delete_workload(&workload_id, &namespace)
            .await
        {
            Ok(_) => HttpResponse::build(StatusCode::NO_CONTENT).body("Remove successfully"),
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => {
                    HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found")
                }
                _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal Server Error"),
            },
        }
    }
}

pub fn get_services() -> Scope {
    web::scope("/workload")
        .service(
            web::resource("/{namespace}/{workload_id}")
                .route(web::delete().to(WorkloadController::delete_workload))
                .route(web::get().to(WorkloadController::get_workload))
                .route(web::patch().to(WorkloadController::patch_workload)),
        )
        .service(
            web::resource("/{namespace}")
                .route(web::put().to(WorkloadController::put_workload))
                .route(web::get().to(WorkloadController::get_all_workloads)),
        )
}
#[cfg(test)]
mod tests {
    use super::WorkloadController;
    use crate::external_api::workload::model::Workload;
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test, web, App,
    };
    use serde_json;
    use std::str::from_utf8;

    #[actix_web::test]
    async fn test_put_and_get() {
        // create temp app http for test
        let app = test::init_service(
            App::new().service(
                web::scope("/workload")
                    .service(
                        web::resource("/").route(web::put().to(WorkloadController::put_workload)),
                    )
                    .service(
                        web::resource("/{workload_id}")
                            .route(web::get().to(WorkloadController::get_workload)),
                    ),
            ),
        )
        .await;
        // create put request to insert workload
        let payload = r#"{
            "name":"nginx",
            "environment" : ["NGINX_PROXY=false"],
            "ports": ["80", "443"],
            "uri":"http://localhost"
        }"#;
        let req_put = test::TestRequest::put()
            .uri("/workload/")
            .insert_header(ContentType::json())
            .set_payload(payload)
            .to_request();
        let resp_put = test::call_and_read_body(&app, req_put).await;
        let workload: Workload = serde_json::from_str(from_utf8(&resp_put).unwrap()).unwrap();
        // create get request to get workload width id in url
        let path_with_id = format!("/workload/{}", workload.id);
        let req_get = test::TestRequest::get().uri(&path_with_id).to_request();
        let resp_get = test::call_service(&app, req_get).await;
        assert!(resp_get.status() == StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_put_and_delete() {
        // create temp app http for test
        let app = test::init_service(
            App::new().service(
                web::scope("/{namespace}/workload")
                    .service(
                        web::resource("/").route(web::put().to(WorkloadController::put_workload)),
                    )
                    .service(
                        web::resource("/{namespace}/workload/{workload_id}")
                            .route(web::delete().to(WorkloadController::delete_workload)),
                    ),
            ),
        )
        .await;
        // create put request to insert workload
        let payload = r#"{
            "name":"nginx",
            "environment" : ["NGINX_PROXY=false"],
            "ports": ["80", "443"],
            "uri":"http://localhost"
        }"#;
        let req_put = test::TestRequest::put()
            .uri("/workload/")
            .insert_header(ContentType::json())
            .set_payload(payload)
            .to_request();
        let resp_put = test::call_and_read_body(&app, req_put).await;
        let workload: Workload = serde_json::from_str(from_utf8(&resp_put).unwrap()).unwrap();
        // create get request to get workload width id in url
        let path_with_id = format!("/workload/{}", workload.id);
        let req_delete = test::TestRequest::delete().uri(&path_with_id).to_request();
        let resp_delete = test::call_service(&app, req_delete).await;
        assert!(resp_delete.status() == StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_get_all() {
        // create temp app http for test
        let app = test::init_service(App::new().service(web::scope("/workload").service(
            web::resource("/").route(web::get().to(WorkloadController::get_all_workloads)),
        )))
        .await;
        // create get_all request to get all workloads
        let req_get = test::TestRequest::get().uri("/workload/").to_request();
        let resp_get = test::call_service(&app, req_get).await;
        assert!(resp_get.status() == StatusCode::OK);
    }
}
