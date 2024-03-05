use actix_web::{App, HttpServer};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema,
};
use utoipa_swagger_ui::SwaggerUi;

mod routes;



use utoipa_auto_discovery::utoipa_auto_discovery;
// #[utoipa_auto_discovery(paths = "( MODULE_TREE::MODULE_NAME => MODULE_SRC_FILE_PATH ) ; ( MODULE_TREE::MODULE_NAME => MODULE_SRC_FILE_PATH ) ; ... ;")]
#[utoipa_auto_discovery(
    paths = "( crate::routes => ./src/routes.rs )"
)]
#[derive(OpenApi)]
#[openapi(
        // paths(routes::greet, routes::get_all_pets, routes::hello),
        // paths(crate::routes::hello),
        paths(),
        components(schemas(routes::Person)),
        // modifiers(&SecurityAddon)
    )]
pub struct ApiDoc;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let oa = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .service(routes::greet)
            .service(routes::get_all_pets)
            .service(routes::hello)
            .service(SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", oa.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        components.add_security_scheme(
            "basic_auth",
            SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
        );
    }
}
