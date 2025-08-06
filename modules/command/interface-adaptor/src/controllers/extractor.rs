use std::str::FromStr;

use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::http::{HeaderName, StatusCode, request::Parts};
use axum_extra::{TypedHeader, headers::Header};
use jsonwebtoken::{DecodingKey, TokenData, Validation, decode};
use serde::{Deserialize, Serialize};

use command_domain::user::UserId;

pub struct XCustomeAuthorization(pub String);

impl Header for XCustomeAuthorization {
    fn name() -> &'static HeaderName {
        static NAME: HeaderName = HeaderName::from_static("x-custom-authorization");
        &NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum_extra::headers::Error>
    where
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        values
            .next()
            .and_then(|v| v.to_str().ok())
            .map(|s| XCustomeAuthorization(s.to_string()))
            .ok_or_else(axum_extra::headers::Error::invalid)
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<axum::http::HeaderValue>,
    {
        if let Ok(value) = axum::http::HeaderValue::from_str(&self.0) {
            values.extend(std::iter::once(value));
        }
    }
}

// 仮実装
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct AuthorizedUser {
    pub user_id: UserId,
}

impl<S> FromRequestParts<S> for AuthorizedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    // handler メソッドの引数に AuthorizedUser を追加したときはこのメソッドが呼ばれる
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(XCustomeAuthorization(token)) =
            parts.extract::<TypedHeader<XCustomeAuthorization>>().await.map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing authorization header".into(),
                )
            })?;

        let token_data = validate_bearer_token(&token)?;

        let user_id = validate_user_id(&token_data.claims.sub)?;

        Ok(Self { user_id })
    }
}

// 仮実装
// 認証サービスを用いて検証するようにしたい
fn validate_bearer_token(token: &str) -> Result<TokenData<Claims>, (StatusCode, String)> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("test-key".as_ref()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))
}

fn validate_user_id(value: &str) -> Result<UserId, (StatusCode, String)> {
    UserId::from_str(value).map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_bearer_token() {
        let claims = Claims {
            sub: UserId::new().to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret("test-key".as_ref()),
        )
        .unwrap();

        let result = validate_bearer_token(token.as_str());
        assert!(result.is_ok());
    }
}
