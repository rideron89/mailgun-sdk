//! This is a library for interacting with the MailGun API.
//!
//! Full API documentation: [https://documentation.mailgun.com/en/latest/api_reference.html#api-reference](https://documentation.mailgun.com/en/latest/api_reference.html#api-reference)
//!
//! Typically, you will want to create a [`Client`](struct.Client.html) and use the methods available there.
//!
//! ### Send Message Example
//!
//! ```no_run
//! use mailgun_sdk::Client as MailGunClient;
//! use mailgun_sdk::message::{Email, Message, MessageBuilder};
//!
//! let client = MailGunClient::new("YOUR_API_KEY", "YOUR_DOMAIN.com");
//!
//! let from = Email::new(None, "from@host.com");
//! let to = vec![
//!     Email::new(None, "to1@host.com"),
//!     Email::new(None, "to2@host.com"),
//! ];
//!
//! let mut builder = MessageBuilder::new("Subject Line", &from, &to);
//! builder.html(Some("<HTML><h1>Message Body</h1></HTML>"));
//!
//! client.send_message(builder.get_message()).unwrap();
//! ```

extern crate multipart;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

mod client;
mod error;
pub mod message;

const API_BASE_PATH: &'static str = "https://api.mailgun.net/v3";

pub use client::Client;
pub use error::Error;
