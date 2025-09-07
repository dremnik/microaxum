use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};

/// Organization claims extracted from a Clerk v2 session token.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrgContext {
    /// The ID of the organization (e.g. "org_123").
    pub id: String,
    /// The slug of the organization (e.g. "org-slug").
    pub slug: String,
    /// The role of the user in the organization (e.g. "admin").
    pub role: String,
    /// The names of the permissions the user has in the organization.
    pub permissions: Vec<String>,
    /// Feature-permission map: binary bitmask values for each permission in `permissions`.
    pub feature_permission_map: Vec<i64>,
}

/// Data made available to handlers once a caller is authenticated.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserContext {
    /// User ID (Clerk subject ID).
    pub id: String,
    /// Global roles from a custom claim.
    pub roles: Vec<String>,
    /// Optional organization context if the session token included org claims.
    pub org: Option<OrgContext>,
}

pub async fn inject_user_context(mut req: Request<Body>, next: Next) -> Response {
    let user_context = UserContext {
        id: "user_dummy".to_string(),
        roles: vec!["dummy_role".to_string()],
        org: Some(OrgContext {
            id: "org_dummy".to_string(),
            slug: "dummy-org".to_string(),
            role: "dummy_admin".to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
            feature_permission_map: vec![1, 2],
        }),
    };
    req.extensions_mut().insert(user_context);

    next.run(req).await
}
