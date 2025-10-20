use crate::prelude::{SmtpClient, SmtpError};
use lettre::message::header::ContentType;
use lettre::message::Attachment;
use lettre::Message;
use std::borrow::Cow;
use tracing::info;

pub struct GatePassSendMailRequest {
    pub sender: Cow<'static, str>,
    pub recipient: Cow<'static, str>,
    pub gate_pass_email_html: Cow<'static, str>,
    pub logo_bytes: Vec<u8>,
    pub qr_code_bytes: Vec<u8>,
}

pub trait GatePassMailer {
    async fn send_gate_pass_email(&self, request: GatePassSendMailRequest)
    -> Result<(), SmtpError>;
}

impl GatePassMailer for SmtpClient {
    async fn send_gate_pass_email(
        &self,
        request: GatePassSendMailRequest,
    ) -> Result<(), SmtpError> {
        let logo_inline_attachment = Attachment::new_inline("logo_id".to_string()).body(
            request.logo_bytes,
            ContentType::parse("image/x-icon").unwrap(),
        );

        let qr_code_inline_attachment = Attachment::new_inline("qr_code_id".to_string()).body(
            request.qr_code_bytes.clone(),
            ContentType::parse("image/png").unwrap(),
        );
        let qr_code_attachment = Attachment::new(String::from("qr_code.png")).body(
            request.qr_code_bytes,
            ContentType::parse("image/png").unwrap(),
        );

        let message = Message::builder()
            .from(request.sender.parse()?)
            .to(request.recipient.parse()?)
            .subject("Тимчасова перепустка 242 ЦПП")
            .multipart(
                lettre::message::MultiPart::mixed()
                    .multipart(lettre::message::MultiPart::alternative().singlepart(
                        lettre::message::SinglePart::html(request.gate_pass_email_html.to_string()),
                    ))
                    .singlepart(logo_inline_attachment)
                    .singlepart(qr_code_inline_attachment)
                    .singlepart(qr_code_attachment),
            )?;

        self.send(message).await?;
        info!("Sent Gate Pass email");
        Ok(())
    }
}
