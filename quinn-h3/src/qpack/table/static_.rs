// This is only here because qpack is new and quinn no uses it yet.
// TODO remove allow dead code
#![allow(dead_code)]

use std::borrow::Cow;
use std::collections::HashMap;

use super::field::HeaderField;

#[allow(unused_macros)]
macro_rules! decl_fields {
    [ $( ($key:expr, $value:expr) ),* ] => {
        [
            $(
            HeaderField {
                name: Cow::Borrowed($key),
                value: Cow::Borrowed($value)
            },
        )* ]
    }
}

const PREDEFINED_HEADERS: [HeaderField; 99] = decl_fields![
    (b":authority", b""),
    (b":path", b"/"),
    (b"age", b"0"),
    (b"content-disposition", b""),
    (b"content-length", b"0"),
    (b"cookie", b""),
    (b"date", b""),
    (b"etag", b""),
    (b"if-modified-since", b""),
    (b"if-none-match", b""),
    (b"last-modified", b""),
    (b"link", b""),
    (b"location", b""),
    (b"referer", b""),
    (b"set-cookie", b""),
    (b":method", b"CONNECT"),
    (b":method", b"DELETE"),
    (b":method", b"GET"),
    (b":method", b"HEAD"),
    (b":method", b"OPTIONS"),
    (b":method", b"POST"),
    (b":method", b"PUT"),
    (b":scheme", b"http"),
    (b":scheme", b"https"),
    (b":status", b"103"),
    (b":status", b"200"),
    (b":status", b"304"),
    (b":status", b"404"),
    (b":status", b"503"),
    (b"accept", b"*/*"),
    (b"accept", b"application/dns-message"),
    (b"accept-encoding", b"gzip, deflate, br"),
    (b"accept-ranges", b"bytes"),
    (b"access-control-allow-headers", b"cache-control"),
    (b"access-control-allow-headers", b"content-type"),
    (b"access-control-allow-origin", b"*"),
    (b"cache-control", b"max-age=0"),
    (b"cache-control", b"max-age=2592000"),
    (b"cache-control", b"max-age=604800"),
    (b"cache-control", b"no-cache"),
    (b"cache-control", b"no-store"),
    (b"cache-control", b"public, max-age=31536000"),
    (b"content-encoding", b"br"),
    (b"content-encoding", b"gzip"),
    (b"content-type", b"application/dns-message"),
    (b"content-type", b"application/javascript"),
    (b"content-type", b"application/json"),
    (b"content-type", b"application/x-www-form-urlencoded"),
    (b"content-type", b"image/gif"),
    (b"content-type", b"image/jpeg"),
    (b"content-type", b"image/png"),
    (b"content-type", b"text/css"),
    (b"content-type", b"text/html; charset=utf-8"),
    (b"content-type", b"text/plain"),
    (b"content-type", b"text/plain;charset=utf-8"),
    (b"range", b"bytes=0-"),
    (b"strict-transport-security", b"max-age=31536000"),
    (
        b"strict-transport-security",
        b"max-age=31536000; includesubdomains"
    ),
    (
        b"strict-transport-security",
        b"max-age=31536000; includesubdomains; preload"
    ),
    (b"vary", b"accept-encoding"),
    (b"vary", b"origin"),
    (b"x-content-type-options", b"nosniff"),
    (b"x-xss-protection", b"1; mode=block"),
    (b":status", b"100"),
    (b":status", b"204"),
    (b":status", b"206"),
    (b":status", b"302"),
    (b":status", b"400"),
    (b":status", b"403"),
    (b":status", b"421"),
    (b":status", b"425"),
    (b":status", b"500"),
    (b"accept-language", b""),
    (b"access-control-allow-credentials", b"FALSE"),
    (b"access-control-allow-credentials", b"TRUE"),
    (b"access-control-allow-headers", b"*"),
    (b"access-control-allow-methods", b"get"),
    (b"access-control-allow-methods", b"get, post, options"),
    (b"access-control-allow-methods", b"options"),
    (b"access-control-expose-headers", b"content-length"),
    (b"access-control-request-headers", b"content-type"),
    (b"access-control-request-method", b"get"),
    (b"access-control-request-method", b"post"),
    (b"alt-svc", b"clear"),
    (b"authorization", b""),
    (
        b"content-security-policy",
        b"script-src 'none'; object-src 'none'; base-uri 'none'"
    ),
    (b"early-data", b"1"),
    (b"expect-ct", b""),
    (b"forwarded", b""),
    (b"if-range", b""),
    (b"origin", b""),
    (b"purpose", b"prefetch"),
    (b"server", b""),
    (b"timing-allow-origin", b"*"),
    (b"upgrade-insecure-requests", b"1"),
    (b"user-agent", b""),
    (b"x-forwarded-for", b""),
    (b"x-frame-options", b"deny"),
    (b"x-frame-options", b"sameorigin")
];

lazy_static! {
    static ref PREDEFINED_HEADERS_MAP: HashMap<Cow<'static, [u8]>, (usize, &'static HeaderField)> =
        PREDEFINED_HEADERS
            .iter()
            .enumerate()
            .map(|(idx, field)| (field.name.clone(), (idx, field)))
            .collect();
}

pub struct StaticTable {}

impl StaticTable {
    pub fn get(index: usize) -> Option<&'static HeaderField> {
        if index > Self::count() - 1 {
            return None;
        }
        PREDEFINED_HEADERS.get(index)
    }

    pub fn count() -> usize {
        PREDEFINED_HEADERS.len()
    }

    pub fn find(name: &[u8]) -> Option<&'static (usize, &'static HeaderField)> {
        PREDEFINED_HEADERS_MAP.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * https://tools.ietf.org/html/draft-ietf-quic-qpack-05
     *  3.1.  Static Table
     *  [...]
     *  Note the QPACK static table is indexed from 0, whereas the HPACK
     *  static table is indexed from 1.
     */
    #[test]
    fn test_static_table_index_is_0_based() {
        assert_eq!(
            StaticTable::get(0),
            Some(&HeaderField::new(":authority", ""))
        );
    }

    #[test]
    fn test_static_table_is_full() {
        assert_eq!(StaticTable::count(), 99);
    }

    #[test]
    fn test_static_table_can_get_field() {
        assert_eq!(
            StaticTable::get(98),
            Some(&HeaderField::new("x-frame-options", "sameorigin"))
        );
    }

    #[test]
    fn invalid_index() {
        assert_eq!(StaticTable::get(99), None);
    }

    #[test]
    fn find_by_name() {
        assert_eq!(
            StaticTable::find(b"last-modified"),
            Some(&(10usize, &HeaderField::new("last-modified", "")))
        );
        assert_eq!(StaticTable::find(b"does-not-exist"), None);
    }
}