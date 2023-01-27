///
pub const FORBIDDEN_FORWARD_HEADERS: [&str; 6] = [
    "connection",
    "transfer-encoding",
    "host",
    "connection",
    "origin",
    "referer",
];

pub const FORBIDDEN_PASS_HEADERS: [&str; 9] = [
    "vary",
    "connection",
    "transfer-encoding",
    "access-control-allow-headers",
    "access-control-allow-methods",
    "access-control-expose-headers",
    "access-control-max-age",
    "access-control-request-headers",
    "access-control-request-method",
];

// common defaults
pub const DEFAULT_FORWARD_HEADERS: [&str; 5] = [
    "accept-encoding",
    "accept-language",
    "sec-websocket-extensions",
    "sec-websocket-key",
    "sec-websocket-version",
];

pub const DEFAULT_PASS_HEADERS: [&str; 3] = ["content-encoding", "content-length", "last-modified"];

// defaults if the client provides a cache key
pub const DEFAULT_CACHE_FORWARD_HEADERS: [&str; 3] =
    ["if-modified-since", "if-none-match", "cache-control"];

pub const DEFAULT_CACHE_PASS_HEADERS: [&str; 2] = ["cache-control", "etag"];
