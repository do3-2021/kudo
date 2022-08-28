use crate::external_api::interface::ActixAppState;

use super::{model::Pagination, model::WorkloadDTO, model::WorkloadError, service};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, Scope};
pub struct WorkloadController {}

impl WorkloadController {
    pub fn get_services(&self) -> Scope {
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

    /// `get_workload` is an async function that handle **/workload/\<namespace>/<workload_id>** route (GET)
    /// # Description:
    /// * Get a workload by id and namespace
    /// # Arguments:
    ///
    /// * `params`: web::Path<(String, String)> - The first Path parameter is the namespace and the second the workload id.
    pub async fn get_workload(
        params: web::Path<(String, String)>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let (namespace, workload_id) = params.into_inner();

        let mut workload_service = service::WorkloadService::new(&data.etcd_address)
            .await
            .map_err(|err| {
                let mut http_response = HttpResponse::InternalServerError();
                match err {
                    WorkloadError::Etcd(msg) => http_response.body(format!("Etcd error: {} ", msg)),
                    _ => http_response.body("Internal Server Error"),
                }
            })
            .unwrap();

        match workload_service
            .get_workload(&workload_id, &namespace)
            .await
        {
            Ok(workload) => match serde_json::to_string(&workload) {
                Ok(json) => HttpResponse::build(StatusCode::OK).body(json),
                Err(err) => HttpResponse::InternalServerError().body(format!(
                    "Error while converting the workload to json string : {}",
                    err
                )),
            },
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => {
                    HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found")
                }
                WorkloadError::JsonToWorkload(msg) => {
                    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(format!(
                        "Error while converting JSON string to workload : {}",
                        msg
                    ))
                }
                _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal Server Error"),
            },
        }
    }
    /// `put_workload` is an async function that handle **/workload/\<namespace>** route (PUT)
    /// # Description:
    /// * Create a new workload
    /// # Arguments:
    ///
    /// * `namespace`: web::Path<String> - This is the namespace that the workload will be created in.
    /// * `body`: web::Json<WorkloadDTO> - Contain all information required to create the workload.
    pub async fn put_workload(
        namespace: web::Path<String>,
        body: web::Json<WorkloadDTO>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut workload_service = service::WorkloadService::new(&data.etcd_address)
            .await
            .map_err(|err| {
                let mut http_response = HttpResponse::InternalServerError();
                match err {
                    WorkloadError::Etcd(msg) => http_response.body(format!("Etcd error: {} ", msg)),
                    _ => http_response.body("Internal Server Error"),
                }
            })
            .unwrap();

        let workload_dto = body.into_inner();
        match workload_service
            .create_workload(workload_dto, &namespace)
            .await
        {
            Ok(workload) => match serde_json::to_string(&workload) {
                Ok(json) => HttpResponse::build(StatusCode::CREATED).body(json),
                Err(err) => HttpResponse::InternalServerError().body(format!(
                    "Error while converting the workload to json: {}",
                    err
                )),
            },
            Err(e) => match e {
                WorkloadError::Etcd(msg) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("Etcd error: {} ", msg)),
                WorkloadError::NameAlreadyExists(name) => HttpResponse::build(StatusCode::CONFLICT)
                    .body(format!("Workload with name {} already exists", name)),
                WorkloadError::WorkloadToJson(msg) => HttpResponse::InternalServerError().body(
                    format!("Error while converting the workload to json: {}", msg),
                ),
                _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal Server Error"),
            },
        }
    }

    /// `get_all_workloads` is an async function that handle **/workload/\<namespace>** route (GET)
    /// # Description:
    /// * Get all workload in the namespace
    /// # Arguments:
    ///
    /// * `namespace`: The namespace of the workloads you want to retrieve.
    /// * `pagination`: Option<web::Query<Pagination>>
    pub async fn get_all_workloads(
        namespace: web::Path<String>,
        pagination: Option<web::Query<Pagination>>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut workload_service = service::WorkloadService::new(&data.etcd_address)
            .await
            .map_err(|err| {
                let mut http_response = HttpResponse::InternalServerError();
                match err {
                    WorkloadError::Etcd(msg) => http_response.body(format!("Etcd error: {} ", msg)),
                    _ => http_response.body("Internal Server Error"),
                }
            })
            .unwrap();

        match pagination {
            Option::Some(pagination) => {
                let workloads = workload_service
                    .get_all_workloads(pagination.limit, pagination.offset, &namespace)
                    .await;
                web::Json(workloads)
            }
            Option::None => {
                let workloads = workload_service.get_all_workloads(0, 0, &namespace).await;
                web::Json(workloads)
            }
        }
    }
    /// `patch_workload` is an asynchronous function that handle **/workload/\<namespace>/<workload_id>** route (PATCH)
    /// # Description:
    /// * Update a workload
    /// # Arguments:
    ///
    /// * `params`: web::Path<(String, String)> - The first Path parameter is the namespace and the second the workload id.
    /// * `body`: web::Json<WorkloadDTO> - Contain all information required to create the workload.
    pub async fn patch_workload(
        params: web::Path<(String, String)>,
        body: web::Json<WorkloadDTO>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut workload_service = service::WorkloadService::new(&data.etcd_address)
            .await
            .map_err(|err| {
                let mut http_response = HttpResponse::InternalServerError();
                match err {
                    WorkloadError::Etcd(msg) => http_response.body(format!("Etcd error: {} ", msg)),
                    _ => http_response.body("Internal Server Error"),
                }
            })
            .unwrap();

        let (namespace, workload_id) = params.into_inner();
        let workload_dto = body.into_inner();

        match workload_service
            .update_workload(workload_dto, &workload_id, &namespace)
            .await
        {
            Ok(workload) => match serde_json::to_string(&workload) {
                Ok(json) => HttpResponse::build(StatusCode::CREATED).body(json),
                Err(err) => HttpResponse::InternalServerError().body(format!(
                    "Error while converting the workload to json: {}",
                    err
                )),
            },
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => {
                    HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found")
                }
                WorkloadError::JsonToWorkload(msg) => HttpResponse::InternalServerError().body(
                    format!("Error while converting the workload to json: {}", msg),
                ),
                _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal Server Error"),
            },
        }
    }

    /// `delete_workload` is an async function that handle **/workload/\<namespace>/<workload_id>** route (DELETE)
    /// # Description:
    /// * Delete a workload
    ///
    /// # Arguments:
    ///
    /// * `params`: web::Path<(String, String)> - The first Path parameter is the namespace and the second the workload id.
    pub async fn delete_workload(
        params: web::Path<(String, String)>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut workload_service = service::WorkloadService::new(&data.etcd_address)
            .await
            .map_err(|err| {
                let mut http_response = HttpResponse::InternalServerError();
                match err {
                    WorkloadError::Etcd(msg) => http_response.body(format!("Etcd error: {} ", msg)),
                    _ => http_response.body("Internal Server Error"),
                }
            })
            .unwrap();
        let (namespace, workload_id) = params.into_inner();

        workload_service
            .delete_workload(&workload_id, &namespace)
            .await;

        HttpResponse::build(StatusCode::NO_CONTENT).body("Remove successfully")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::WorkloadController;
//     use actix_web::{
//         http::{header::ContentType, StatusCode},
//         test, web, App,
//     };

// #[actix_web::test]
// async fn test_put_and_get() {
//     // create temp app http for test
//     let app = test::init_service(
//         App::new().service(
//             web::scope("/workload")
//                 .service(
//                     web::resource("/{namespace}")
//                         .route(web::put().to(WorkloadController::put_workload)),
//                 )
//                 .service(
//                     web::resource("/{namespace}/{workload_id}")
//                         .route(web::get().to(WorkloadController::get_workload)),
//                 ),
//         ),
//     )
//     .await;
//     // create put request to insert workload
//     let payload = r#"{
//             "name":"postgres",
//             "environment" : ["NGINX_PROXY=false"],
//             "ports": [{"source":80, "destination":443}],
//             "uri":"http://localhost"
//         }"#;
//     let req_put = test::TestRequest::put()
//         .uri("/workload/default")
//         .insert_header(ContentType::json())
//         .set_payload(payload)
//         .to_request();
//     test::call_service(&app, req_put).await;
//     // create get request to get workload width name in url
//     let path_with_id = "/workload/default/postgres";
//     let req_get = test::TestRequest::get().uri(path_with_id).to_request();
//     let resp_get = test::call_service(&app, req_get).await;
//     println!("{:?}", resp_get.status());
//     dbg!(&resp_get);
//     assert!(resp_get.status() == StatusCode::OK);
//     // remove the workload
//     let req_delete = test::TestRequest::delete().uri(path_with_id).to_request();
//     test::call_service(&app, req_delete).await;
// }

//     #[actix_web::test]
//     async fn test_put_and_delete() {
//         // create temp app http for test
//         let app = test::init_service(
//             App::new().service(
//                 web::scope("/workload")
//                     .service(
//                         web::resource("/{namespace}")
//                             .route(web::put().to(WorkloadController::put_workload)),
//                     )
//                     .service(
//                         web::resource("/{namespace}/{workload_id}")
//                             .route(web::delete().to(WorkloadController::delete_workload)),
//                     ),
//             ),
//         )
//         .await;
//         // create put request to insert workload
//         let payload = r#"{
//             "name":"redis",
//             "environment" : ["NGINX_PROXY=false"],
//             "ports": [{"source":80, "destination":443}],
//             "uri":"http://localhost"
//         }"#;
//         let req_put = test::TestRequest::put()
//             .uri("/workload/default")
//             .insert_header(ContentType::json())
//             .set_payload(payload)
//             .to_request();
//         test::call_service(&app, req_put).await;
//         // create get request to get workload width name in url
//         let path_with_id = "/workload/default/redis";
//         let req_delete = test::TestRequest::delete().uri(path_with_id).to_request();
//         let resp_delete = test::call_service(&app, req_delete).await;
//         assert!(resp_delete.status() == StatusCode::NO_CONTENT);
//     }

//     #[actix_web::test]
//     async fn test_get_all() {
//         // create temp app http for test
//         let app = test::init_service(
//             App::new().service(
//                 web::scope("/workload").service(
//                     web::resource("/{namespace}")
//                         .route(web::get().to(WorkloadController::get_all_workloads)),
//                 ),
//             ),
//         )
//         .await;
//         // create get_all request to get all workloads
//         let req_get = test::TestRequest::get()
//             .uri("/workload/default")
//             .to_request();
//         let resp_get = test::call_service(&app, req_get).await;
//         println!("{:?}", resp_get.status());
//         assert!(resp_get.status() == StatusCode::OK);
//     }
// }
