//! Web socket hooks.

use std::cell::RefCell;
use std::rc::Rc;

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::WebSocketError;
use js_sys::wasm_bindgen::JsError;
use dioxus::prelude::*;

pub use gloo_net::websocket::Message;

#[derive(Clone, Copy)]
pub struct WebSocketHandle {
    ws_write: Signal<Rc<RefCell<SplitSink<WebSocket, Message>>>>,
    state: Signal<websocket::State>,
    message: Signal<String>,
    message_bytes: Signal<Vec<u8>>,
}

/// Opens a web socket connection at the specified `url`. The connection is closed when the enclosing scope is destroyed
/// or when [`WebSocketHandle::close`] is called.
pub fn use_web_socket(url: &str) -> Result<WebSocketHandle, JsError> {
    let state = use_signal(|| websocket::State::Closed);
    let mut message = use_signal(|| String::new());
    let mut message_bytes = use_signal(|| Vec::new());

    let ws = WebSocket::open(url)?;
    let (write, mut read) = ws.split();


    spawn(async move {
        while let Some(next) = read.next().await {
            if let Ok(m) = next {
                match m {
                    Message::Text(t) => message.set(t),
                    Message::Bytes(b) => message_bytes.set(b),
                }
            }
        }
    });

    Ok(WebSocketHandle {
        ws_write: use_signal(|| Rc::new(RefCell::new(write))),
        state,
        message,
        message_bytes,
    })
}

impl WebSocketHandle {
    // TODO: solve this issue
    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn send(&mut self, message: Message) -> Result<(), WebSocketError> {
        self.ws_write.write().borrow_mut().send(message).await
    }

    pub fn status(self) -> Signal<websocket::State> {
        self.state
    }

    pub fn message_texts(self) -> Signal<String> {
        self.message
    }

    pub fn message_bytes(self) -> Signal<Vec<u8>> {
        self.message_bytes
    }

    /// NOTE: Not yet implemented due to technical reasons.
    pub fn close(self) {
        unimplemented!();
    }
}
