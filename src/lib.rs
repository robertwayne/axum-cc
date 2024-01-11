#![doc = include_str!("../README.md")]
pub mod mime;

use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use axum_core::response::Response;
use futures_core::ready;
use http::{
    header::{CACHE_CONTROL, CONTENT_TYPE},
    HeaderValue, Request,
};
use pin_project_lite::pin_project;
use tower_layer::Layer;
use tower_service::Service;

pub use crate::mime::MimeType;

const DEFAULT_MIME_TYPES: [MimeType; 6] = [
    MimeType::CSS,
    MimeType::JS,
    MimeType::SVG,
    MimeType::WEBP,
    MimeType::WOFF2,
    MimeType::PNG,
];

/// A [`tower::Layer`] that sets `Cache-Control` headers on responses.
///
/// See
/// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control>
/// for more information.
#[derive(Debug, Default, Clone, Copy)]
pub struct CacheControlLayer<'a> {
    mime_types: &'a [MimeType],
    max_age: Duration,
    // TODO: add support for remaining directives
}

impl<'a> CacheControlLayer<'a> {
    /// Create a new `CacheControlLayer` with the default configuration.
    ///
    /// The default configuration sets `Cache-Control` headers for the following
    /// MIME types:
    ///
    /// - `text/css`
    /// - `application/javascript`
    /// - `image/svg+xml`
    /// - `image/webp`
    /// - `font/woff2`
    ///
    /// The `max-age` value is set to 1 year.
    ///
    /// If you wish to set your own MIME types and/or `max-age` value, use
    /// [`CacheControlLayer::with_mime_types`] and/or
    /// [`CacheControlLayer::with_max_age`].
    ///
    /// As these are builder methods, you can chain them together:
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use axum_cc::{CacheControlLayer, MimeType};
    ///
    /// let layer = CacheControlLayer::default() // empty headers
    ///    .with_mime_types(&[MimeType::CSS])
    ///    .with_max_age(Duration::from_secs(86400));
    /// ```
    pub fn new() -> Self {
        Self {
            mime_types: &DEFAULT_MIME_TYPES,
            max_age: Duration::from_secs(60 * 60 * 24 * 365),
        }
    }

    /// Set the MIME types that should have `Cache-Control` headers set.
    pub fn with_mime_types(mut self, mime_types: &'a [MimeType]) -> Self {
        self.mime_types = mime_types;
        self
    }

    /// Set the `max-age` value for the `Cache-Control` header.
    pub fn with_max_age(mut self, max_age: impl Into<Duration>) -> Self {
        self.max_age = max_age.into();
        self
    }
}

impl<'a, S> Layer<S> for CacheControlLayer<'a> {
    type Service = CacheControl<'a, S>;

    fn layer(&self, inner: S) -> Self::Service {
        CacheControl {
            inner,
            layer: *self,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheControl<'a, S> {
    inner: S,
    layer: CacheControlLayer<'a>,
}

impl<'a, S, T, U> Service<Request<T>> for CacheControl<'a, S>
where
    S: Service<Request<T>, Response = Response<U>>,
    U: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<'a, S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<T>) -> Self::Future {
        let response_future = self.inner.call(req);

        ResponseFuture {
            response_future,
            layer: self.layer,
        }
    }
}

pin_project! {
    pub struct ResponseFuture<'a, F> {
        #[pin]
        response_future: F,
        layer: CacheControlLayer<'a>,
    }
}

impl<'a, F, B, E> Future for ResponseFuture<'a, F>
where
    F: Future<Output = Result<Response<B>, E>>,
    B: Default,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let mut response: Response<B> = ready!(this.response_future.poll(cx))?;

        if let Some(content_type) = response.headers().get(CONTENT_TYPE) {
            let mime = MimeType::from(content_type);

            if this.layer.mime_types.contains(&mime) {
                let value = format!("public, max-age={}", this.layer.max_age.as_secs());

                if let Ok(value) = HeaderValue::from_str(&value) {
                    response.headers_mut().insert(CACHE_CONTROL, value);
                }
            }
        }

        Poll::Ready(Ok(response))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheControlError {
    InvalidMaxAge,
    InvalidMimeType,
}

impl fmt::Display for CacheControlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheControlError::InvalidMaxAge => write!(f, "invalid max-age value"),
            CacheControlError::InvalidMimeType => write!(f, "invalid MIME type"),
        }
    }
}

impl std::error::Error for CacheControlError {}
