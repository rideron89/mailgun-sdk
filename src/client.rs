use crate::error;
use crate::message;

/// Utility for interacting with the MailGun API.
///
/// You can create a `Client` via the `new` method.
///
/// ### Example
///
/// ```rust
/// use mailgun_sdk::Client as MailGunClient;
/// use mailgun_sdk::message::Message;
///
/// let client = MailGunClient::new("YOUR_API_KEY", "YOUR_DOMAIN.com");
/// ```
#[derive(Debug)]
pub struct Client<'a> {
    api_key: &'a str,
    client: reqwest::Client,
    domain: &'a str,
}

impl<'a> Client<'a> {
    /// Create a new MailGun client. Requires an `api_key` and `domain`.
    ///
    /// You can find your API key in the *Security* tab under the *Account* section of the
    /// MailGun Control Panel.
    ///
    /// **Important**: Make sure you keep your API key secret.
    #[allow(dead_code)]
    pub fn new(api_key: &'a str, domain: &'a str) -> Client<'a> {
        Client {
            api_key,
            client: reqwest::Client::new(),
            domain,
        }
    }

    /// Get the API key.
    pub fn api_key(&self) -> &'a str {
        self.api_key
    }

    /// Get the web client.
    pub fn client(&'a self) -> &'a reqwest::Client {
        &self.client
    }

    /// Get the domain.
    pub fn domain(&self) -> &'a str {
        self.domain
    }
}

// Methods for the Methods API.
impl<'a> Client<'a> {
    /// Send a message to MailGun.
    ///
    /// Refer to the [`message`](message) module documentation.
    ///
    /// Panics if no body is set. Make sure you set either the [`text`](message/struct.MessageBuilder.html#method.text)
    /// or [`html`](message/struct.MessageBuilder.html#method.html) field of the message before trying to
    /// send it.
    pub fn send_message(&self, message: &'a message::Message) -> Result<message::SendMessageResponse, error::Error> {
        message::send_message_with_client(&self, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_new() {
        let client = Client::new("api_key", "domain");

        assert_eq!("api_key", client.api_key);
        assert_eq!("domain", client.domain);
    }

    #[test]
    fn send_message() {
        let from = message::Email::new(None, "test@test.com");
        let to = vec![
            message::Email::new(None, "test@test.com"),
        ];
        let mut message_builder = message::MessageBuilder::new("Subject Line", &from, &to);
        message_builder.text(Some("Message body"));
        message_builder.option_testmode(Some("yes"));

        let _client = Client::new("", "");

        // let response = client.send_message(message_builder.get_message()).unwrap();

        // Sample API success response:
        // {
        //     "message": "Queued. Thank you.",
        //     "id": "<20111114174239.25659.5817@samples.mailgun.org>"
        // }

        // match response {
        //     message::SendMessageResponse::Success { id: _, message } => {
        //         assert_eq!("Queued. Thank you.", message);
        //     },
        //     message::SendMessageResponse::Failure { message: _ } => {
        //         panic!("Received API error");
        //     }
        // }
    }
}
