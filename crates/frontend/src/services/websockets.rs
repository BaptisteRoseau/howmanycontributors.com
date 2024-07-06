use crate::error::Error;
use tracing::debug;
use web_sys::wasm_bindgen::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};

use super::requests::API_ROOT;

pub struct ServiceWebsocket {
    ws: WebSocket,
}

impl ServiceWebsocket {
    /// Creates a websocket for the given path.
    ///
    /// You can then use the websocket to send and receive messages using callbacks.
    ///
    /// ## Example
    ///
    /// ```rs
    /// use web_sys::wasm_bindgen::prelude::*;
    /// use web_sys::wasm_bindgen::JsCast;
    /// use web_sys::{ErrorEvent, MessageEvent, WebSocket};
    ///
    /// pub fn try_websocket() -> Result<(), JsValue> {
    ///     // Callback to handle messages
    ///     let onmessage_callback = move |e: MessageEvent| {
    ///         if let Ok(message) = e.data().dyn_into::<js_sys::JsString>() {
    ///             println!("Received: {}", message);
    ///         }
    ///     };
    ///
    ///     // Callback to handle errors
    ///     let onerror_callback = move |e: ErrorEvent| {
    ///         println!("Received: {:?}", e.message());
    ///     };
    ///
    ///     // Callback to handle the connection closing
    ///     let onclose_callback = move |e: CloseEvent| {
    ///         println!("Connection closed by server: {:?}", e.reason());
    ///     };
    ///
    ///     let _ = ServiceWebsocket::new("/myendpoint")
    ///         .unwrap()
    ///         .set_onmessage(onmessage_callback)
    ///         .set_error(onerror_callback)
    ///         .set_onclose(onclose_callback);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(path: &str) -> Result<ServiceWebsocket, Error> {
        assert!(
            API_ROOT.starts_with("http"),
            "API_ROOT must start with 'http'"
        );
        let url = format!("{}{}", API_ROOT.replacen("http", "ws", 1).as_str(), path);
        debug!("WS {}", url);
        match WebSocket::new(url.as_str()) {
            Ok(ws) => Ok(ServiceWebsocket { ws }),
            Err(_) => Err(Error::WebSocket),
        }
    }

    /// Adds a callback to the websocket that will be called when a message is received.
    ///
    /// ## Example
    ///
    /// ```rs
    /// ServiceWebsocket::new("/myendpoint")
    /// .unwrap()
    /// .set_onmessage(move |e: MessageEvent| {
    ///     if let Ok(message) = e.data().dyn_into::<js_sys::JsString>() {
    ///         println!("Received: {}", message);
    ///     }
    /// });
    /// ```
    pub fn set_onmessage<T>(&mut self, callback: T) -> &mut Self
    where
        T: FnMut(MessageEvent) + 'static,
    {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(MessageEvent)>);
        self.ws
            .set_onmessage(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
        self
    }

    /// Adds a callback to the websocket that will be called on error.
    /// ## Example
    ///
    /// ```rs
    /// ServiceWebsocket::new("/myendpoint")
    /// .unwrap()
    /// .set_error(move |e: ErrorEvent| {
    ///     println!("Received: {:?}", e.message());
    /// });
    /// ```
    pub fn set_error<T>(&mut self, callback: T) -> &mut Self
    where
        T: FnMut(ErrorEvent) + 'static,
    {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(ErrorEvent)>);
        self.ws.set_onerror(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
        self
    }

    /// Adds a callback to the websocket that will be called when the connection is closed.
    /// ## Example
    ///
    /// ```rs
    /// ServiceWebsocket::new("/myendpoint")
    /// .unwrap()
    /// .set_onclose(move |e: CloseEvent| {
    ///     println!("Connection closed by server: {:?}", e.reason());
    /// });
    /// ```
    pub fn set_onclose<T>(&mut self, callback: T) -> &mut Self
    where
        T: FnMut(CloseEvent) + 'static,
    {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(CloseEvent)>);
        self.ws.set_onclose(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
        self
    }

    /// Closes the websocket. No error checking is done.
    pub fn is_open(&self) -> bool {
        self.ws.ready_state() != web_sys::WebSocket::OPEN
    }

    /// Closes the websocket. No error checking is done.
    pub fn close(&mut self) {
        match self.ws.ready_state() {
            web_sys::WebSocket::CLOSING | web_sys::WebSocket::CLOSED => (),
            _ => {
                let _ = self.ws.close();
            }
        }
    }

    pub fn inner(&self) -> &WebSocket {
        &self.ws
    }

    pub fn into_inner(self) -> WebSocket {
        self.ws
    }
}
