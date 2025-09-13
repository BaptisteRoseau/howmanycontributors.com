use super::websockets::ServiceWebsocket;
use crate::error::Error;
use crate::models::ContributorsChunk;

use tracing::debug;
use web_sys::MessageEvent;
use web_sys::js_sys;
use web_sys::wasm_bindgen::prelude::*;

/// Get decks filtered by author
pub fn get_dependencies<T>(link: &str, mut callback: T) -> Result<ServiceWebsocket, Error>
where
    T: FnMut(ContributorsChunk) + 'static,
{
    let mut ws = ServiceWebsocket::new(format!("/dependencies?link={link}").as_str())?;
    ws.set_onmessage(move |e: MessageEvent| {
        if let Ok(message) = e.data().dyn_into::<js_sys::JsString>() {
            debug!("Received Dependency Chunk: {}", message);
            if let Some(msg) = message.as_string()
                && let Ok(chunk) = ContributorsChunk::try_from(msg.as_str()) {
                    callback(chunk);
                }
        }
    });
    Ok(ws)
}

impl TryFrom<&str> for ContributorsChunk {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let Some(split) = value.split_once(':') else {
            return Err(Error::InvalidChunkFormat(value.to_string()));
        };

        let contributors = split
            .1
            .parse::<usize>()
            .map_err(|_| Error::InvalidChunkFormat(value.to_string()))?;
        let path = split.0.to_string();
        Ok(ContributorsChunk { path, contributors })
    }
}
