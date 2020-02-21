//! Model for messages sent to or back from the MailGun API.
//!
//! Full API documentation: [https://documentation.mailgun.com/en/latest/api-sending.html](https://documentation.mailgun.com/en/latest/api-sending.html)
//!
//! A message may be created with just a `subject`, `from` recipient, and `to` recipient list.
//! However, you must have either an HTML or raw text body before sending the message to MailGun.
//!
//! You should use [`MessageBuilder`](struct.MessageBuilder.html) to build and modify your message before sending it to MailGun.
//!
//! ### Example
//!
//! ```rust
//! use mailgun_sdk::message::{Email, MessageBuilder};
//!
//! let from = Email::new(Some("Sender"), "sender@domain.com");
//! let to = vec![
//!     Email::new(Some("Recipient 1"), "recipient1@domain.com"),
//!     Email::new(Some("Recipient 2"), "recipient2@domain.com"),
//! ];
//!
//! let mut builder = MessageBuilder::new("Subject Line", &from, &to);
//! builder.html(Some("<html><h1>Your Email</h1></html>"));
//! builder.text(Some("Your Email"));
//!
//! let message = builder.get_message();
//! ```

use crate::error;
use multipart::client::lazy::Multipart;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

/// Represents a custom data object to be sent with the message.
type MessageJsonData<'a> = HashMap<&'a str, &'a str>;

/// A message that can be sent or retrieved from MailGun.
///
/// You should use [`MessageBuilder`](struct.MessageBuilder.html) to build and modify your message before sending it to MailGun.
///
/// You can find the meaning of all the fields here:
/// [https://documentation.mailgun.com/en/latest/api-sending.html#sending](https://documentation.mailgun.com/en/latest/api-sending.html#sending)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message<'a> {
    from: Email<'a>,
    to: EmailList<'a>,
    cc: Option<EmailList<'a>>,
    bcc: Option<EmailList<'a>>,
    subject: &'a str,
    text: Option<&'a str>,
    html: Option<&'a str>,
    amp_html: Option<&'a str>,
    attachment: Option<AttachmentList<'a>>,
    inline: Option<AttachmentList<'a>>,
    template: Option<&'a str>,
    template_version: Option<&'a str>,
    template_text: Option<bool>,
    option_tag: Option<&'a str>,
    option_dkim: Option<&'a str>,
    option_deliverytime: Option<&'a str>,
    option_testmode: Option<&'a str>,
    option_tracking: Option<&'a str>,
    option_tracking_clicks: Option<&'a str>,
    option_tracking_opens: Option<bool>,
    option_require_tls: Option<bool>,
    option_skip_verification: Option<bool>,
    custom_headers: Option<HashMap<&'a str, &'a str>>,
    custom_data: Option<MessageJsonData<'a>>,
    recipient_variables: Option<MessageJsonData<'a>>,
}

impl<'a> Message<'a> {
    /// Create a new message from a subject, from `Email`, and list of to `Email`s.
    #[allow(dead_code)]
    pub fn new(subject: &'a str, from: &'a Email, to: &'a Vec<Email>) -> Message<'a> {
        let from = from.clone();
        let to = to.clone();
        let subject = subject.clone();

        let to = EmailList { emails: to };

        Message {
            from,
            to,
            cc: None,
            bcc: None,
            subject,
            text: None,
            html: None,
            amp_html: None,
            attachment: None,
            inline: None,
            template: None,
            template_version: None,
            template_text: None,
            option_tag: None,
            option_dkim: None,
            option_deliverytime: None,
            option_testmode: None,
            option_tracking: None,
            option_tracking_clicks: None,
            option_tracking_opens: None,
            option_require_tls: None,
            option_skip_verification: None,
            custom_headers: None,
            custom_data: None,
            recipient_variables: None,
        }
    }

    /// Get the message's `from` field.
    pub fn from(&self) -> Email<'a> {
        self.from.clone()
    }

    /// Get the message's `to` field.
    pub fn to(&self) -> Vec<Email> {
        self.to.emails.clone()
    }

    /// Get the message's `cc` field.
    pub fn cc(&self) -> Option<Vec<Email>> {
        match &self.cc {
            Some(cc) => Some(cc.emails.clone()),
            None => None,
        }
    }

    /// Get the message's `bcc` field.
    pub fn bcc(&self) -> Option<Vec<Email>> {
        match &self.bcc {
            Some(bcc) => Some(bcc.emails.clone()),
            None => None,
        }
    }

    /// Get the message's `subject` field.
    pub fn subject(&self) -> &'a str {
        self.subject
    }

    /// Get the message's `text` field.
    pub fn text(&self) -> Option<&'a str> {
        self.text
    }

    /// Get the message's `html` field.
    pub fn html(&self) -> Option<&'a str> {
        self.html
    }

    /// Get the message's `amp-html` field.
    pub fn amp_html(&self) -> Option<&'a str> {
        self.amp_html
    }

    /// Get the message's `attachment` field.
    pub fn attachment(&self) -> Option<Vec<Attachment>> {
        match &self.attachment {
            Some(attachment) => Some(attachment.attachments.clone()),
            None => None,
        }
    }

    /// Get the message's `inline` field.
    pub fn inline(&self) -> Option<Vec<Attachment>> {
        match &self.inline {
            Some(inline) => Some(inline.attachments.clone()),
            None => None,
        }
    }

    /// Get the message's `template` field.
    pub fn template(&self) -> Option<&'a str> {
        self.template
    }

    /// Get the message's `t:version` field.
    pub fn template_version(&self) -> Option<&'a str> {
        self.template_version
    }

    /// Get the message's `t:text` field.
    pub fn template_text(&self) -> Option<bool> {
        self.template_text
    }

    /// Get the message's `o:tag` field.
    pub fn option_tag(&self) -> Option<&'a str> {
        self.option_tag
    }

    /// Get the message's `o:dkim` field.
    pub fn option_dkim(&self) -> Option<&'a str> {
        self.option_dkim
    }

    /// Get the message's `o:deliverytime` field.
    pub fn option_deliverytime(&self) -> Option<&'a str> {
        self.option_deliverytime
    }

    /// Get the message's `o:testmode` field.
    pub fn option_testmode(&self) -> Option<&'a str> {
        self.option_testmode
    }

    /// Get the message's `o:tracking` field.
    pub fn option_tracking(&self) -> Option<&'a str> {
        self.option_tracking
    }

    /// Get the message's `o:tracking-clicks` field.
    pub fn option_tracking_clicks(&self) -> Option<&'a str> {
        self.option_tracking_clicks
    }

    /// Get the message's `o:tracking-opens` field.
    pub fn option_tracking_opens(&self) -> Option<bool> {
        self.option_tracking_opens
    }

    /// Get the message's `o:require-tls` field.
    pub fn option_require_tls(&self) -> Option<bool> {
        self.option_require_tls
    }

    /// Get the message's `o:skip_verification` field.
    pub fn option_skip_verification(&self) -> Option<bool> {
        self.option_skip_verification
    }

    /// Get the message's custom headers list.
    pub fn custom_headers(&self) -> Option<HashMap<&'a str, &'a str>> {
        self.custom_headers.clone()
    }

    /// Get the message's custom data object.
    pub fn custom_data(&self) -> Option<MessageJsonData<'a>> {
        self.custom_data.clone()
    }

    /// Ge the message's recipient variables object.
    pub fn recipient_variables(&self) -> Option<MessageJsonData<'a>> {
        self.recipient_variables.clone()
    }
}

impl<'a> Message<'a> {
    /// Return the message as a multipart form.
    pub fn as_form(&self) -> Result<Multipart, error::Error> {
        let mut multipart = Multipart::new();

        multipart.add_text("from", self.from.to_string());
        multipart.add_text("to", self.to.to_string());

        if let Some(cc) = &self.cc {
            multipart.add_text("cc", cc.to_string());
        }

        if let Some(bcc) = &self.bcc {
            multipart.add_text("bcc", bcc.to_string());
        }

        multipart.add_text("subject", self.subject);

        if let Some(text) = self.text {
            multipart.add_text("text", text);
        }

        if let Some(amp_html) = self.amp_html {
            multipart.add_text("amp-html", amp_html);
        }

        if let Some(attachment_list) = &self.attachment {
            for attachment in attachment_list.attachments() {
                multipart.add_file(attachment.name, attachment.file_path);
            }
        }

        if let Some(inline_list) = &self.inline {
            for inline in inline_list.attachments() {
                multipart.add_file(inline.name, inline.file_path);
            }
        }

        if let Some(template) = self.template {
            multipart.add_text("template", template);
        }

        if let Some(template_version) = self.template_version {
            multipart.add_text("t:version", template_version);
        }

        if let Some(template_text) = self.template_text {
            if template_text {
                multipart.add_text("t:text", "yes");
            }
        }

        if let Some(option_tag) = self.option_tag {
            multipart.add_text("o:tag", option_tag);
        }

        if let Some(option_dkim) = self.option_dkim {
            multipart.add_text("o:dkim", option_dkim);
        }

        if let Some(option_deliverytime) = self.option_deliverytime {
            multipart.add_text("o:deliverytime", option_deliverytime);
        }

        if let Some(option_testmode) = self.option_testmode {
            multipart.add_text("o:testmode", option_testmode);
        }

        if let Some(option_tracking) = self.option_tracking {
            multipart.add_text("o:tracking", option_tracking);
        }

        if let Some(option_tracking_clicks) = self.option_tracking_clicks {
            multipart.add_text("o:tracking-clicks", option_tracking_clicks);
        }

        if let Some(option_tracking_opens) = self.option_tracking_opens {
            let option_tracking_opens = if option_tracking_opens {
                "yes"
            } else {
                "no"
            };

            multipart.add_text("o:tracking-opens", option_tracking_opens);
        }

        if let Some(option_require_tls) = self.option_require_tls {
            let option_require_tls = if option_require_tls {
                "yes"
            } else {
                "no"
            };

            multipart.add_text("o:require-tls", option_require_tls);
        }

        if let Some(option_skip_verification) = self.option_skip_verification {
            let option_skip_verification = if option_skip_verification {
                "yes"
            } else {
                "no"
            };

            multipart.add_text("o:skip-verification", option_skip_verification);
        }

        if let Some(custom_headers) = &self.custom_headers {
            for (key, value) in custom_headers {
                let name = format!("h:{}", key);

                multipart.add_text(name, *value);
            }
        }

        if let Some(custom_data) = &self.custom_data {
            for (key, value) in custom_data {
                let name = format!("v:{}", key);

                multipart.add_text(name, *value);
            }
        }

        if let Some(recipient_variables) = &self.recipient_variables {
            match serde_json::to_string(recipient_variables) {
                Ok(recipient_variables) => {
                    multipart.add_text("recipient-variables", recipient_variables);
                },
                Err(error) => {
                    return Err(error::Error::MessageError(error));
                }
            }
        }

        Ok(multipart)
    }
}

/// Facilitates building a message to be sent to MailGun.
///
/// Api documentation: [https://documentation.mailgun.com/en/latest/api-sending.html#sending](https://documentation.mailgun.com/en/latest/api-sending.html#sending)
pub struct MessageBuilder<'a> {
    message: Message<'a>,
}

impl<'a> MessageBuilder<'a> {
    /// Create a new message builder instance. This is the recommended method of creating a
    /// message to send to MailGun.
    pub fn new(subject: &'a str, from: &'a Email, to: &'a Vec<Email>) -> MessageBuilder<'a> {
        let message = Message::new(subject, from, to);

        MessageBuilder { message }
    }

    /// Return a reference to the underlying [`Message`](struct.Message.html).
    pub fn get_message(&self) -> &Message<'a> {
        &self.message
    }

    /// Email address for From header.
    pub fn from(&mut self, from:&'a Email) -> &mut MessageBuilder<'a> {
        self.message.from = from.clone();

        self
    }

    /// Email address of the recipient(s).
    pub fn to(&mut self, to: &'a Vec<Email>) -> &mut MessageBuilder<'a> {
        self.message.to = EmailList { emails: to.clone() };

        self
    }

    /// Email address of the CC recipients.
    pub fn cc(&mut self, cc: Option<&'a Vec<Email>>) -> &mut MessageBuilder<'a> {
        match cc {
            Some(cc) => self.message.cc = Some(EmailList { emails: cc.clone() }),
            None => self.message.cc = None,
        }

        self
    }

    /// Email addres of the BCC recipients.
    pub fn bcc(&mut self, bcc: Option<&'a Vec<Email>>) -> &mut MessageBuilder<'a> {
        match bcc {
            Some(bcc) => self.message.bcc = Some(EmailList { emails: bcc.clone() }),
            None => self.message.bcc = None,
        }

        self
    }

    /// Message subject.
    pub fn subject(&mut self, subject: &'a str) -> &mut MessageBuilder<'a> {
        self.message.subject = subject.clone();

        self
    }

    /// Raw text body of the message.
    pub fn text(&mut self, text: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.text = text.clone();

        self
    }

    /// HTML body of the message.
    pub fn html(&mut self, html: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.html = html.clone();

        self
    }

    /// [AMP](https://developers.google.com/gmail/ampemail/) part of the message. Please follow
    /// google [guidelines](https://developers.google.com/gmail/ampemail/) to compose and send
    /// AMP emails.
    pub fn amp_html(&mut self, amp_html: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.amp_html = amp_html.clone();

        self
    }

    /// File attachment(s).
    pub fn attachment(&mut self, attachment: &Attachment<'a>) -> &mut MessageBuilder<'a> {
        if self.message.attachment.is_none() {
            self.message.attachment = Some(AttachmentList { attachments: vec![] });
        }

        if let Some(ref mut attachment_list) = self.message.attachment {
            attachment_list.push(attachment);
        }

        self
    }

    /// Attachment(s) with inline disposition. Can be used to send inline images.
    pub fn inline(&mut self, inline: Option<&'a Vec<Attachment>>) -> &mut MessageBuilder<'a> {
        match inline {
            Some(inline) => self.message.inline = Some(AttachmentList { attachments: inline.clone() }),
            None => self.message.inline = None,
        }

        self
    }

    /// Name of a template stored via [template API](https://documentation.mailgun.com/en/latest/api-templates.html#api-templates).
    pub fn template(&mut self, template: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.template = template.clone();

        self
    }

    /// Set a specific version of the template.
    pub fn template_version(&mut self, template_version: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.template_version = template_version.clone();

        self
    }

    /// Set to `true` to have the rendered template in the text part of the message when using
    /// template sending.
    pub fn template_text(&mut self, template_text: Option<bool>) -> &mut MessageBuilder<'a> {
        self.message.template_text = template_text.clone();

        self
    }

    /// Tag string. See [Tagging](https://documentation.mailgun.com/en/latest/user_manual.html#tagging)
    /// for more information.
    pub fn option_tag(&mut self, option_tag: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.option_tag = option_tag.clone();

        self
    }

    /// Set to `true` to enable DKIM signatures. Use `false` to force disabling DKIM.
    pub fn option_dkim(&mut self, option_dkim: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.option_dkim = option_dkim.clone();

        self
    }

    /// Desired time of delivery. See [Date Format](https://documentation.mailgun.com/en/latest/api-intro.html#date-format).
    /// Note: Messages can be scheduled for a maximum of 3 days in the future.
    pub fn option_deliverytime(&mut self, option_deliverytime: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.option_deliverytime = option_deliverytime.clone();

        self
    }

    /// Set to `true` to send in test mode. See [Test Mode](https://documentation.mailgun.com/en/latest/user_manual.html#manual-testmode).
    pub fn option_testmode(&mut self, option_testmode: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.option_testmode = option_testmode.clone();

        self
    }

    /// Set to `true` to enable tracking. Set to `false` to force disable tracking.
    pub fn option_tracking(&mut self, option_tracking: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.option_tracking = option_tracking.clone();

        self
    }

    /// Toggle click tracking. Set to `yes`, `no`, `true`, `false`, or `htmlonly`.
    pub fn option_tracking_clicks(&mut self, option_tracking_clicks: Option<&'a str>) -> &mut MessageBuilder<'a> {
        self.message.option_tracking_clicks = option_tracking_clicks.clone();

        self
    }

    /// Set to `true` to enable opens tracking. Set to `false` to force disable opens tracking.
    pub fn option_tracking_opens(&mut self, option_tracking_opens: Option<bool>) -> &mut MessageBuilder<'a> {
        self.message.option_tracking_opens = option_tracking_opens.clone();

        self
    }

    /// Set to `true` to force sending the message over a TLS connection. If TLS cannot be
    /// established, MailGun will not deliver the message. If set to `false`, MailGun will try to
    /// upgrade the connection, but will deliver the message over a plaintext SMTP connection if
    /// it cannot.
    pub fn option_require_tls(&mut self, option_require_tls: Option<bool>) -> &mut MessageBuilder<'a> {
        self.message.option_require_tls = option_require_tls.clone();

        self
    }

    /// Set to `true` to skip certificate and hostname verification when trying to establish a
    /// TLS connection. If set to `false`, MailGun will only send the message if the certificate
    /// and hostname can be verified.
    pub fn option_skip_verification(&mut self, option_skip_verification: Option<bool>) -> &mut MessageBuilder<'a> {
        self.message.option_skip_verification = option_skip_verification.clone();

        self
    }

    /// List of custom headers to be sent as MIME headers with the message.
    pub fn custom_headers(&mut self, custom_headers: Option<HashMap<&'a str, &'a str>>) -> &mut MessageBuilder<'a> {
        self.message.custom_headers = custom_headers.clone();

        self
    }

    /// Attach custom JSON data to the message. See [Attaching Data to Messages](https://documentation.mailgun.com/en/latest/user_manual.html#manual-customdata).
    pub fn custom_data(&mut self, custom_data: Option<MessageJsonData<'a>>) -> &mut MessageBuilder<'a> {
        self.message.custom_data = custom_data.clone();

        self
    }

    /// Recipient variables sent with batch sending. Each key should be a recipient and each value
    /// should be a JSON-encoded dictionary of variables. See [Batch Sending](https://documentation.mailgun.com/en/latest/user_manual.html#batch-sending).
    pub fn recipient_variables(&mut self, recipient_variables: Option<MessageJsonData<'a>>) -> &mut MessageBuilder<'a> {
        self.message.recipient_variables = recipient_variables.clone();

        self
    }
}

/// Email address.
///
/// If the `name` field is set, the full email address will be used/shown.
/// Example: `Name <address@domain.com>`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Email<'a> {
    name: Option<&'a str>,
    address: &'a str,
}

impl<'a> Email<'a> {
    /// Create a new email from an address and optional name field.
    #[allow(dead_code)]
    pub fn new(name: Option<&'a str>, address: &'a str) -> Email<'a> {
        Email { name, address }
    }

    /// Set the display name portion of the email.
    pub fn name(&mut self, name: Option<&'a str>) {
        self.name = name;
    }

    /// Set the full address portion of the email.
    pub fn address(&mut self, address: &'a str) {
        self.address = address;
    }

    /// Return a string representation of the email.
    ///
    /// If the `name` field is set, returns in the format of `Name <email@host.com>`; otherwise,
    /// returns in the format of `email@host.com`.
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self.name {
            Some(name) => {
                format!("{} <{}>", name, self.address)
            },
            None => {
                format!("{}", self.address)
            }
        }
    }
}

impl<'a> Serialize for Email<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Wrapper around a list of emails.
#[derive(Clone, Debug, Deserialize)]
struct EmailList<'a> {
    #[serde(borrow)]
    emails: Vec<Email<'a>>,
}

impl<'a> EmailList<'a> {
    fn to_string(&self) -> String {
        self.emails.iter().map(|email| email.to_string()).collect::<Vec<String>>().join(",")
    }
}

impl<'a> Serialize for EmailList<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let emails = self.emails
            .iter()
            .map(|attachment| attachment.to_string())
            .collect::<Vec<String>>()
            .join(",");

        serializer.serialize_str(&emails)
    }
}

/// File attachment that can be sent with a message.
#[derive(Clone, Debug, Deserialize)]
pub struct Attachment<'a> {
    name: &'a str,
    file_path: &'a str,
}

impl<'a> Attachment<'a> {
    /// Create a new attachment.
    pub fn new(name: &'a str, file_path: &'a str) -> Attachment<'a> {
        Attachment { name, file_path }
    }

    /// Get the attachment's name.
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Get the attachment's file path.
    pub fn file_path(&self) -> &'a str {
        self.file_path
    }

    /// Set the name of the attachment.
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    /// Set the file path of the attachment.
    pub fn set_file_path(&mut self, file_path: &'a str) {
        self.file_path = file_path;
    }

    /// Return a string representation of the attachment.
    ///
    /// Returns in the format of `@{file_name}:{file_path}`.
    pub fn to_string(&self) -> String {
        format!("@{}:{}", self.name, self.file_path)
    }
}

impl<'a> Serialize for Attachment<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Wrapper around a list of attachments.
#[derive(Clone, Debug, Deserialize)]
struct AttachmentList<'a> {
    #[serde(borrow)]
    attachments: Vec<Attachment<'a>>,
}

impl<'a> AttachmentList<'a> {
    /// Return the list of attachments.
    fn attachments(&self) -> Vec<Attachment<'a>> {
        self.attachments.clone()
    }

    /// Appends an `Attachment` to the end of the list.
    fn push(&mut self, attachment: &Attachment<'a>) {
        self.attachments.push(attachment.clone());
    }
}

impl<'a> Serialize for AttachmentList<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let attachments = self.attachments
            .iter()
            .map(|attachment| attachment.to_string())
            .collect::<Vec<String>>()
            .join(",");

        serializer.serialize_str(&attachments)
    }
}

/// Response sent back from MailGun after sending a message.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
// We need to use `String` instead of `&'a str` here, because `reqwest` requires an owned
// response. We cannot use borrowed fields.
pub enum SendMessageResponse {
    Success {
        message: String,
        id: String,
    },

    Failure {
        message: String,
    },
}

/// Send a message to MailGun with an existing [`Client`](../struct.Client.html).
///
/// Panics if no body is set. Make sure you set either the [`text`](message/struct.MessageBuilder.html#method.text)
/// or [`html`](message/struct.MessageBuilder.html#method.html) field of the message before trying to
/// send it.
pub fn send_message_with_client<'a>(client: &crate::Client, message: &'a Message) -> Result<SendMessageResponse, error::Error<'a>> {
    if message.text().is_none() && message.html().is_none() {
        panic!("No message body is set");
    }

    let url = format!("{}/{}/messages", crate::API_BASE_PATH, client.domain());

    let mut request = client.client()
        .post(&url)
        .basic_auth("api", Some(client.api_key()));

    if message.attachment().is_none() && message.inline().is_none() {
        request = request.form(&message);
    } else {
        let mut form_params = message
            .as_form()?
            .prepare()
            .map_err(|error| error::Error::MessageParamsError(error))?;

        request = request.header("Content-Type", &format!("multipart/form-data; boundary={}", form_params.boundary()));

        let mut body = String::new();
        form_params.to_body().read_to_string(&mut body)
            .map_err(|error| error::Error::MessageBodyError(error))?;

        request = request.body(body);
    }

    let mut response = request.send()
        .map_err(|error| error::Error::Unknown(error.to_string()))?;

    let response_text = response.text().map_err(|_| {
        error::Error::Unknown(String::from("Unable to read response"))
    })?;

    if &response_text == "Forbidden" {
        return Err(error::Error::ApiForbiddenError);
    }

    serde_json::from_str::<SendMessageResponse>(&response_text)
        .map_err(|error| error::Error::Unknown(error.to_string()))
        .and_then(|response| {
            match response {
                SendMessageResponse::Success { id: _, message: _ } => {
                    Ok(response)
                },
                SendMessageResponse::Failure { message: _ } => {
                    Err(error::Error::SendMessageError(response))
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use std::io::Read;
    use super::*;

    #[test]
    fn message_new() {
        let from = Email { name: None, address: "test@test.com" };
        let to = vec![
            Email { name: None, address: "test@test.com" },
            Email { name: None, address: "test@test.com" },
        ];
        let subject = "Subject line";

        let message = Message::new(subject, &from, &to);

        assert_eq!(from, message.from);
        assert_eq!(2, message.to.emails.len());
        assert_eq!(to.get(0).unwrap(), message.to.emails.get(0).unwrap());
        assert_eq!(to.get(1).unwrap(), message.to.emails.get(1).unwrap());
    }

    #[test]
    fn message_builder_new() {
        let from = Email { name: None, address: "test@test.com" };
        let to = vec![
            Email { name: None, address: "test@test.com" },
            Email { name: None, address: "test@test.com" },
        ];
        let subject = "Subject line";
        let text = "Raw Message body";

        let mut message_builder = MessageBuilder::new(subject, &from, &to);
        message_builder.text(Some(text));

        let message = message_builder.get_message();

        assert_eq!(from.address, message.from().address);
        assert_eq!(subject, message.subject());
        assert_eq!(Some(text), message.text());
    }

    #[test]
    fn message_as_form() {
        let from = Email { name: None, address: "test@test.com" };
        let to = vec![
            Email { name: None, address: "test@test.com" },
            Email { name: None, address: "test@test.com" },
        ];
        let subject = "Subject line";

        let message_builder = MessageBuilder::new(subject, &from, &to);

        let message = message_builder.get_message();

        let mut form_params = message.as_form().unwrap();
        let mut form_params = form_params.prepare().unwrap();

        let mut body = form_params.to_body();
        let mut body_buffer = String::new();
        body.read_to_string(&mut body_buffer).unwrap();

        {
            let boundary = form_params.boundary();
            let content_length = form_params.content_len().unwrap();
            assert!(boundary.len() > 0);
            assert!(content_length > 0);
        }
    }

    #[test]
    fn attachment_list_serialize() {
        #[derive(Serialize)]
        struct Test<'a> {
            attachments: AttachmentList<'a>,
        }

        let attachments = AttachmentList {
            attachments: vec![
                Attachment { name: "name1", file_path: "path1" },
                Attachment { name: "name2", file_path: "path2" },
                Attachment { name: "name3", file_path: "path3" },
            ]
        };

        let test = Test { attachments };
        let result = serde_urlencoded::to_string(test).unwrap();
        assert_eq!("attachments=%40name1%3Apath1%2C%40name2%3Apath2%2C%40name3%3Apath3", result);
    }

    #[test]
    fn email_list_serialize() {
        #[derive(Serialize)]
        struct Test<'a> {
            emails: EmailList<'a>,
        }

        let emails = EmailList {
            emails: vec![
                Email { name: None, address: "test1@test.com" },
                Email { name: None, address: "test2@test.com" },
            ]
        };

        let test = Test { emails };
        let result = serde_urlencoded::to_string(test).unwrap();
        assert_eq!("emails=test1%40test.com%2Ctest2%40test.com", result);
    }

    #[test]
    fn message_serialize() {
        let from = Email::new(None, "test@test.com");
        let to = vec![
            Email::new(None, "test1@test.com"),
            Email::new(None, "test2@test.com"),
        ];

        let message_builder = MessageBuilder::new("Subject Line", &from, &to);
        let message = message_builder.get_message();

        match serde_urlencoded::to_string(message) {
            Ok(result) => {
                assert_eq!(r#"from=test%40test.com&to=test1%40test.com%2Ctest2%40test.com&subject=Subject+Line"#, result);
            },
            Err(error) => {
                eprintln!("{:#?}", error);
                panic!("Could not serialize message");
            }
        }
    }

    #[test]
    fn email_new() {
        let full = Email::new(Some("Name"), "test@test.com");
        let partial = Email::new(None, "test@test.com");

        assert_eq!(Some("Name"), full.name);
        assert_eq!("test@test.com", full.address);
        assert_eq!(None, partial.name);
        assert_eq!("test@test.com", partial.address);
    }

    #[test]
    fn email_to_string() {
        let full = Email { name: Some("Name"), address: "test@test.com" };
        let partial = Email { name: None, address: "test@test.com" };

        assert_eq!("Name <test@test.com>", full.to_string());
        assert_eq!("test@test.com", partial.to_string());
    }
}
