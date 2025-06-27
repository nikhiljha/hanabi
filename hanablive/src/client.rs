use std::pin::Pin;
use std::task::{Context, Poll};
use futures_core::Stream;
use reqwest::header::HeaderValue;
use reqwest_websocket::{Error, RequestBuilderExt, WebSocket};
use thiserror::Error;
use crate::messages;
use futures_util::sink::SinkExt;
use futures_util::{Sink, StreamExt};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("request failed")]
    RequestFailed(#[from] reqwest::Error),

    #[error("websocket request failed")]
    WebsocketRequestFailed(#[from] reqwest_websocket::Error),

    #[error("websocket parse error")]
    WebsocketParseError(#[from] messages::WebsocketParseError),

    #[error("stream map error")]
    StreamMapError,

    #[error("login failed: `{0}`")]
    LoginFailed(String),
}

pub struct Client {
    client: reqwest::Client,
    base_url: String,
    cookie: Option<HeaderValue>,
    websocket: Option<WebSocket>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::default(),
            base_url: "https://hanab.live".to_string(),
            cookie: None,
            websocket: None,
        }
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), ClientError> {
        // POST to /login with username and password in FormData
        let res = self.client.post(format!("{}/login", self.base_url))
            .form(&[("username", username), ("password", password), ("version", "6356")])
            .send()
            .await?;

        // Check for a Set-Cookie header
        if let Some(cookie) = res.headers().get("Set-Cookie") {
            self.cookie = Some(cookie.clone());
        } else {
            return Err(ClientError::LoginFailed(res.text().await.unwrap_or_default()))
        }

        Ok(())
    }

    pub async fn connect_ws(&mut self) -> Result<(), ClientError> {
        // GET to /ws with the cookie
        let res = self.client.get(format!("{}/ws", self.base_url))
            .header("Cookie", self.cookie.clone().unwrap())
            .upgrade()
            .send()
            .await?;

        // Check for a 101 Switching Protocols response
        let websocket = res.into_websocket().await?;
        self.websocket = Some(websocket);
        Ok(())
    }
}

impl Sink<messages::Message> for Client {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.websocket.as_mut().unwrap().poll_ready_unpin(cx).map_err(Into::into)
    }

    fn start_send(mut self: Pin<&mut Self>, item: messages::Message) -> Result<(), Self::Error> {
        self.websocket.as_mut().unwrap().start_send_unpin(reqwest_websocket::Message::Text(item.to_websocket_message().unwrap()))
            .map_err(Into::into)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.websocket.as_mut().unwrap().poll_flush_unpin(cx).map_err(Into::into)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.websocket.as_mut().unwrap().poll_close_unpin(cx).map_err(Into::into)
    }
}


impl Stream for Client {
    type Item = Result<messages::Message, ClientError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match self.websocket.as_mut().unwrap().poll_next_unpin(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Ready(Some(Err(error))) => return Poll::Ready(Some(Err(error.into()))),
                Poll::Ready(Some(Ok(reqwest_websocket::Message::Text(message)))) => {
                    return match messages::Message::from_websocket_message(&message) {
                        Ok(message) => Poll::Ready(Some(Ok(message))),
                        Err(e) => {
                            Poll::Ready(Some(Err(ClientError::from(e))))
                        }
                    }
                }
                // Something weird! Probably okay to ignore.
                _ => {}
            }
        }
    }
}
