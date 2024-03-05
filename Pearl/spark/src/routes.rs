use actix_web::{get, post, web::{self, Json}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use utoipa::{openapi::schema, ToSchema};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[utoipa::path(
    responses(
        (status = 200, description = "List all Pets", body = [ListPetsDTO])
    )
)]

#[get("/pets")]
async fn get_all_pets(req: HttpRequest, store: web::Data<AppState>) -> impl Responder {
    format!("Hello !")
}

#[post("person")]
// #[utoipa::path(
//     request_body = Person,
//     response(

//     )
// )]
pub async fn hello(body: Json<Person>) -> impl Responder {
    let person = body.into_inner();
    let resp = HttpResponse::Ok().json(person);
	return resp;
}

struct AppState {
	
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Person {
    #[schema(example = "johndoe", required = true)]
    name: String,
    age: i32
}
