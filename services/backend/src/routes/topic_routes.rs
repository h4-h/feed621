use axum::{extract::Path, response::IntoResponse, routing::get};

pub(crate) fn topic_routes() -> axum::Router {
    axum::Router::new()
        .route("/topics", 
             get(find_topic_by_name)
            .post(create_topic)
        )
        .route("/topics/:id/subscribers", get(get_topic_posts))
        .route("/topics/:id/posts", get(get_topic_posts))
        
        .route("/topics/:id", 
            get(get_topic_by_id)
            .patch(update_topic)
            .delete(delete_topic)
        )
}

async fn create_topic(

) -> impl IntoResponse {
    "NYI: create topic"
}

async fn get_topic_by_id(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: get topic by id"
}

async fn find_topic_by_name(
    
) -> impl IntoResponse {
    "NYI: find topic by name"
}

async fn get_topic_posts(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: get topic posts"
}

async fn get_topic_subscribers(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: get topic subscribers"
}

async fn update_topic(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: update topic"
}

async fn delete_topic(
    Path(_id): Path<i64>,
) -> impl IntoResponse {
    "NYI: delete topic"
}
