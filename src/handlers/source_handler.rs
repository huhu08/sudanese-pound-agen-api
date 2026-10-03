use axum::Json;
use serde_json::Value;

use crate::services::sanity_service::
    get_sources_from_sanity;

pub async fn sources()
-> Json<Value> {

    let result =
        get_sources_from_sanity()
            .await
            .unwrap();

    Json(result)
}
