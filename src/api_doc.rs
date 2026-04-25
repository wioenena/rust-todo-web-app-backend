use utoipa::OpenApi;

use crate::routes;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/todos", tags = ["Todos"], api = routes::todos::docs::TodosApi)
    )
)]
pub struct ApiDoc;
