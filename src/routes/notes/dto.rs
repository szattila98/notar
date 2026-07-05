use serde::Deserialize;

#[derive(Deserialize, utoipa_axum::ToSchema)]
pub struct CreateNote {
    pub title: String,
}

#[derive(Deserialize, utoipa_axum::ToSchema)]
pub struct UpdateNote {
    pub title: String,
    pub content: serde_json::Value,
}
