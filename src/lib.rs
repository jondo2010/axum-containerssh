#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use types::*;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "0.5.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthPasswordResponse {
    /// Response is the full HTTP authentication response.
    Status200_ResponseIsTheFullHTTPAuthenticationResponse(models::AuthResponseBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthPubKeyResponse {
    /// Response is the full HTTP authentication response.
    Status200_ResponseIsTheFullHTTPAuthenticationResponse(models::AuthResponseBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthzResponse {
    /// Response is the full HTTP authentication response.
    Status200_ResponseIsTheFullHTTPAuthenticationResponse(models::AuthResponseBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetUserConfigurationResponse {
    /// Response is the entire response from the config server
    Status200_ResponseIsTheEntireResponseFromTheConfigServer(models::ConfigResponseBody),
}

/// API
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Api {
    /// AuthPassword - POST /password
    async fn auth_password(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::PasswordAuthRequest,
    ) -> Result<AuthPasswordResponse, String>;

    /// AuthPubKey - POST /pubkey
    async fn auth_pub_key(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::PublicKeyAuthRequest,
    ) -> Result<AuthPubKeyResponse, String>;

    /// Authz - POST /authz
    async fn authz(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::AuthorizationRequest,
    ) -> Result<AuthzResponse, String>;

    /// GetUserConfiguration - POST /config
    async fn get_user_configuration(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: Option<models::ConfigRequest>,
    ) -> Result<GetUserConfigurationResponse, String>;
}

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod types;

#[cfg(feature = "server")]
pub(crate) mod header;
