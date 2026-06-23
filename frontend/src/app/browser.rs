#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[allow(deprecated)]
#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
fn blob_from_bytes(data: &[u8], mime: &str) -> Result<Blob, String> {
    let uint8 = js_sys::Uint8Array::from(data);
    let parts = js_sys::Array::new();
    parts.push(&uint8.buffer());

    let mut opts = BlobPropertyBag::new();
    #[allow(deprecated)]
    opts.type_(mime);

    Blob::new_with_buffer_source_sequence_and_options(&parts, &opts)
        .map_err(|e| format!("blob error: {e:?}"))
}

#[cfg(target_arch = "wasm32")]
fn blob_from_text(data: &str, mime: &str) -> Result<Blob, String> {
    let parts = js_sys::Array::new();
    parts.push(&JsValue::from_str(data));

    let mut opts = BlobPropertyBag::new();
    #[allow(deprecated)]
    opts.type_(mime);

    Blob::new_with_str_sequence_and_options(&parts, &opts).map_err(|e| format!("blob error: {e:?}"))
}

#[cfg(target_arch = "wasm32")]
fn trigger_download_url(url: &str, filename: &str) -> Result<(), String> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or_else(|| "No document".to_string())?;

    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .map_err(|e| format!("{e:?}"))?
        .dyn_into()
        .map_err(|e| format!("{e:?}"))?;

    anchor.set_href(url);
    anchor.set_download(filename);
    anchor.set_attribute("style", "display:none").ok();

    if let Some(body) = document.body() {
        body.append_child(&anchor).map_err(|e| format!("{e:?}"))?;
        anchor.click();
        body.remove_child(&anchor).map_err(|e| format!("{e:?}"))?;
    } else {
        anchor.click();
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn schedule_revoke(url: String) -> Result<(), String> {
    let window = web_sys::window().ok_or_else(|| "No window".to_string())?;
    let revoke = Closure::once(move || {
        let _ = Url::revoke_object_url(&url);
    });
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            revoke.as_ref().unchecked_ref(),
            60_000,
        )
        .map_err(|e| format!("{e:?}"))?;
    revoke.forget();
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn download_blob(blob: &Blob, filename: &str, revoke: bool) -> Result<String, String> {
    let url = Url::create_object_url_with_blob(blob).map_err(|e| format!("URL error: {e:?}"))?;
    trigger_download_url(&url, filename)?;
    if revoke {
        schedule_revoke(url.clone())?;
    }
    Ok(url)
}

#[cfg(target_arch = "wasm32")]
pub fn download_bytes(data: &[u8], filename: &str, mime: &str) -> Result<(), String> {
    let blob = blob_from_bytes(data, mime)?;
    download_blob(&blob, filename, true).map(|_| ())
}

#[cfg(target_arch = "wasm32")]
pub fn download_text(data: &str, filename: &str, mime: &str) -> Result<(), String> {
    let blob = blob_from_text(data, mime)?;
    download_blob(&blob, filename, true).map(|_| ())
}

#[cfg(target_arch = "wasm32")]
pub fn download_png(data: &[u8], filename: &str) -> Result<(), String> {
    download_bytes(data, filename, "image/png")
}

#[cfg(target_arch = "wasm32")]
pub fn download_svg(svg: &str, filename: &str) -> Result<(), String> {
    download_text(svg, filename, "image/svg+xml")
}

#[cfg(target_arch = "wasm32")]
pub fn download_json(json: &str, filename: &str) -> Result<(), String> {
    download_text(json, filename, "application/json")
}

#[cfg(target_arch = "wasm32")]
pub fn cached_download_callback(
    cached_url: UseStateHandle<Option<String>>,
    rendering: UseStateHandle<bool>,
    filename: &'static str,
    label: &'static str,
    render_bytes: Rc<dyn Fn() -> Result<Vec<u8>, String>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        if *rendering {
            return;
        }
        if let Some(url) = (*cached_url).clone() {
            let _ = trigger_download_url(&url, filename);
            return;
        }

        rendering.set(true);
        let cached_url_done = cached_url.clone();
        let rendering_done = rendering.clone();
        let render_bytes = render_bytes.clone();

        let callback = Closure::once(move || {
            match render_bytes()
                .and_then(|data| blob_from_bytes(&data, "application/zip"))
                .and_then(|blob| download_blob(&blob, filename, false))
            {
                Ok(url) => cached_url_done.set(Some(url)),
                Err(error) => {
                    web_sys::console::error_1(&format!("{label} {error}").into());
                }
            }
            rendering_done.set(false);
        });

        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                0,
            );
            callback.forget();
        } else {
            rendering.set(false);
        }
    })
}

#[cfg(target_arch = "wasm32")]
pub fn upload_json(on_load: yew::Callback<String>) {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    let input: web_sys::HtmlInputElement = match document
        .create_element("input")
        .ok()
        .and_then(|e| e.dyn_into().ok())
    {
        Some(i) => i,
        None => return,
    };

    input.set_type("file");
    input.set_attribute("accept", ".json,application/json").ok();
    input.set_attribute("style", "display:none").ok();

    let body = match document.body() {
        Some(b) => b,
        None => return,
    };
    body.append_child(&input).ok();

    let input_clone = input.clone();
    let on_change = Closure::<dyn FnMut()>::new(move || {
        let files = match input_clone.files() {
            Some(f) => f,
            None => return,
        };
        let file = match files.get(0) {
            Some(f) => f,
            None => return,
        };

        let reader = match web_sys::FileReader::new().ok() {
            Some(r) => r,
            None => return,
        };

        let on_load = on_load.clone();
        let reader_clone = reader.clone();
        let onloadend = Closure::<dyn FnMut()>::new(move || {
            if let Ok(result) = reader_clone.result()
                && let Some(text) = result.as_string()
            {
                on_load.emit(text);
            }
        });

        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
        onloadend.forget();

        reader.read_as_text(&file).ok();

        if let Some(parent) = input_clone.parent_node() {
            parent.remove_child(&input_clone).ok();
        }
    });

    input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
    on_change.forget();

    input.click();
}
