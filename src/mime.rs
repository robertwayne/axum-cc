use std::fmt::Display;

use http::HeaderValue;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MimeType {
    CSS,
    HTML,
    JS,
    SVG,
    TEXT,
    WEBP,
    WOFF2,
    PNG,
}

impl MimeType {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "css" => MimeType::CSS,
            "html" => MimeType::HTML,
            "js" => MimeType::JS,
            "svg" => MimeType::SVG,
            "webp" => MimeType::WEBP,
            "woff2" => MimeType::WOFF2,
            "png" => MimeType::PNG,
            _ => MimeType::TEXT,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MimeType::CSS => "text/css",
            MimeType::HTML => "text/html",
            MimeType::JS => "application/javascript",
            MimeType::SVG => "image/svg+xml",
            MimeType::TEXT => "text/plain",
            MimeType::WEBP => "image/webp",
            MimeType::WOFF2 => "font/woff2",
            MimeType::PNG => "image/png",
        }
    }
}

impl From<&HeaderValue> for MimeType {
    fn from(header: &HeaderValue) -> Self {
        let header = header
            .to_str()
            .unwrap_or_default()
            .split(';')
            .next()
            .unwrap_or_default();

        match header {
            "text/css" => MimeType::CSS,
            "text/html" => MimeType::HTML,
            "application/javascript" => MimeType::JS,
            "image/svg+xml" => MimeType::SVG,
            "text/plain" => MimeType::TEXT,
            "image/webp" => MimeType::WEBP,
            "font/woff2" => MimeType::WOFF2,
            "image/png" => MimeType::PNG,
            _ => MimeType::TEXT,
        }
    }
}

impl Display for MimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
