//! A WebSocket based, asynchronous Twitch chat client.
//!
//! # Example
//! ```no_run
//! #[macro_use]
//! extern crate log;
//!
//! use std::env;
//! use std::error::Error;
//!
//! use futures::future::{join, ready};
//! use futures_util::SinkExt;
//!
//! use tmi_rs::{futures_util::stream::StreamExt, TwitchChatConnection, TwitchClient, TwitchClientConfigBuilder};
//! use tmi_rs::client_messages::ClientMessage;
//! use tmi_rs::event::{ChannelMessageEventData, Event};
//! use tmi_rs::rate_limits::RateLimiterConfig;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     env_logger::init();
//!     let channel = env::var("TWITCH_CHANNEL")?;
//!     let client: TwitchClient = TwitchClientConfigBuilder::default()
//!         .username(env::var("TWITCH_USERNAME")?)
//!         .token(env::var("TWITCH_AUTH")?)
//!         .rate_limiter(RateLimiterConfig::default())
//!         .build()?
//!         .into();
//!
//!     let TwitchChatConnection { mut sender, receiver, error_receiver } = client.connect().await?;
//!
//!     // join a channel
//!     sender.send(ClientMessage::join(channel.clone())).await?;
//!
//!     // process messages and do stuff with the data
//!     let process_messages = async {
//!         let send_result = receiver.filter_map(
//!             |event| {
//!                 info!("{:?}", &event);
//!                 ready(match &*event {
//!                     Event::PrivMsg(event_data) if event_data.message().starts_with("!hello") => {
//!                         // return response message to the stream
//!                         Some(ClientMessage::message(event_data.channel().to_owned(), "Hello World!"))
//!                     }
//!                     _ => None
//!                 })
//!             })
//!             .map(Ok)
//!             .forward(&mut sender).await;
//!
//!         if let Err(e) = send_result { error!("{}", e); }
//!     };
//!
//!     // log any connection errors
//!     let process_errors = error_receiver.for_each(async move |error| {
//!         error!("Connection error: {}", error);
//!     });
//!     join(process_messages, process_errors).await;
//!     Ok(())
//! }
//! ```

#![deny(
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces,
    missing_docs
)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate log;
#[macro_use]
extern crate pin_utils;
#[macro_use]
extern crate smallvec;

use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub use futures_channel;
pub use futures_core;
pub use futures_sink;
pub use futures_util;

pub use client::*;
use client_messages::*;
pub use errors::*;
pub use sender::*;

mod client;
pub mod client_messages;
mod errors;
pub mod event;
pub mod irc;
pub mod irc_constants;
pub mod rate_limits;
pub mod selectors;
mod sender;
pub(crate) mod util;

/// Trait that is used when generically referring to a &str, String, or other type that can be
/// used like a borrowed string
pub trait StringRef: Borrow<str> + Debug + Clone + Hash + Eq + Display {}
impl<T> StringRef for T where T: Borrow<str> + Debug + Clone + Hash + Eq + Display {}
