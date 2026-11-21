//! `content://localhost/<subject>/images/x.png` (on Windows
//! `http://content.localhost/…`): the content's raster images for the
//! webview. SVGs are inlined instead, so they follow the theme.

use std::borrow::Cow;

use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Manager, UriSchemeContext, Wry};

use crate::state::AppState;

pub fn handle(
    ctx: UriSchemeContext<'_, Wry>,
    request: Request<Vec<u8>>,
) -> Response<Cow<'static, [u8]>> {
    let path = percent_decode(request.uri().path().trim_start_matches('/'));
    let asset = ctx
        .app_handle()
        .try_state::<AppState>()
        .and_then(|state| state.service.asset(&path));
    let response = match asset {
        Some((bytes, mime)) => Response::builder()
            .header(header::CONTENT_TYPE, mime)
            .header(header::CACHE_CONTROL, "no-cache")
            .body(Cow::Owned(bytes)),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Cow::Borrowed(&b""[..])),
    };
    response.unwrap_or_else(|_| Response::new(Cow::Borrowed(&b""[..])))
}

/// `%2F` → `/`, `%C3%A1` → `á`; a malformed escape stays as it is.
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
            if let Some(b) = hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::percent_decode;

    #[test]
    fn escapes_are_decoded() {
        assert_eq!(
            percent_decode("bpc-fy1%2Fimages%2Fx.png"),
            "bpc-fy1/images/x.png"
        );
        assert_eq!(percent_decode("a%C3%A1"), "aá");
        assert_eq!(percent_decode("bad%zz"), "bad%zz");
        assert_eq!(percent_decode("end%2"), "end%2");
        assert_eq!(percent_decode("end%2F"), "end/");
    }
}
