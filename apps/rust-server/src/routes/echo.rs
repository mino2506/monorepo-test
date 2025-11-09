use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EchoPayload {
    text: String,
}

pub async fn handler(Json(payload): Json<EchoPayload>) -> Json<EchoPayload> {
    Json(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;

    #[tokio::test]
    async fn echo_returns_same_payload() {
        let req = EchoPayload {
            text: "Hello, World!".into(),
        };

        let Json(res) = handler(Json(req)).await;

        assert_eq!(res.text, "Hello, World!".to_string());
    }
}
