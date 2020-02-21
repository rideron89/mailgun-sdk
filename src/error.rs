use crate::message;
use std::error;
use std::fmt;
use std::io;

/// Wrapper around the various errors the library might experience.
#[derive(Debug)]
pub enum Error<'a> {
    /// Returned when the user does not have access to part (or all) of an API. Typically, this
    /// is thrown when an invalid API key is used.
    ApiForbiddenError,

    /// Returned when serializing part of a [`Message`](message/struct.Message.html) fails.
    MessageError(serde_json::Error),

    /// Returned when the message body could not be formed into a `multipart/form-data` body.
    MessageBodyError(io::Error),

    /// Returned when the message itself could not be formed into a `multipart/form-data` message.
    MessageParamsError(multipart::client::lazy::LazyIoError<'a>),

    /// Returned when MailGun responds with an error when sending a message.
    SendMessageError(message::SendMessageResponse),

    /// Returned for generic errors.
    Unknown(String),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiForbiddenError => write!(f, "API Forbidden Error"),
            Self::MessageError(error) => write!(f, "Message Error: {}", error),
            Self::MessageBodyError(error) => write!(f, "Message Body Error: {}", error),
            Self::MessageParamsError(error) => write!(f, "Message Params Error: {}", error),
            Self::SendMessageError(error) => write!(f, "Send Message Error: {:?}", error),
            Self::Unknown(error) => write!(f, "Unknown Error: {}", error),
        }
    }
}

impl<'a> error::Error for Error<'a> {}
