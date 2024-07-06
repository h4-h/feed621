use aide::{axum::{routing::{get_with, post_with}, ApiRouter, IntoApiResponse}, transform::TransformOperation};
use axum::extract::Path;

pub(crate) fn topic_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route("/topics", 
            post_with(create_topic, create_topic_docs)
            .get_with(find_topic_by_name, find_topic_by_name_docs)
        )
        .api_route("/topics/:id/subscribers", 
            get_with(get_topic_subscribers, get_topic_subscribers_docs)
        )
        .api_route("/topics/:id/posts",
            get_with(get_topic_posts, get_topic_posts_docs)
        )
        .api_route("/topics/:id", 
            get_with(get_topic_by_id, get_topic_by_id_docs)
            .patch_with(update_topic, update_topic_docs)
            .delete_with(delete_topic, delete_topic_docs)
        )
}

async fn create_topic(

) -> impl IntoApiResponse {
    "NYI: create topic"
}

fn create_topic_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create topic route")
        .tag("Topic routes")
        .response::<201, String>()
}

async fn get_topic_by_id(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: get topic by id"
}

fn get_topic_by_id_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get information about topic")
        .tag("Topic routes")
        .response::<200, String>()
}

async fn find_topic_by_name(
    
) -> impl IntoApiResponse {
    "NYI: find topic by name"
}

fn find_topic_by_name_docs(op: TransformOperation) -> TransformOperation {
    op.description("Find topic by name")
        .tag("Topic routes")
        .response::<200, String>()
}

async fn get_topic_posts(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: get topic posts"
}

fn get_topic_posts_docs(op: TransformOperation) -> TransformOperation {
    op.description("")
        .tag("Topic routes")
        .response::<200, String>()
}

async fn get_topic_subscribers(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: get topic subscribers"
}

fn get_topic_subscribers_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get topic subscribers")
        .tag("Topic routes")
        .response::<200, String>()
}

async fn update_topic(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: update topic"
}

fn update_topic_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update topic information")
        .tag("Topic routes")
        .response::<202, String>()
}

async fn delete_topic(
    Path(_id): Path<i64>,
) -> impl IntoApiResponse {
    "NYI: delete topic"
}

fn delete_topic_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete topic")
        .tag("Topic routes")
        .response::<204, String>()
}
