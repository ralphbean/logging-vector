// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(Debug)]
pub struct SelectObjectContentEventStreamUnmarshaller;

impl SelectObjectContentEventStreamUnmarshaller {
    pub fn new() -> Self {
        SelectObjectContentEventStreamUnmarshaller
    }
}
impl aws_smithy_eventstream::frame::UnmarshallMessage
    for SelectObjectContentEventStreamUnmarshaller
{
    type Output = crate::model::SelectObjectContentEventStream;
    type Error = crate::error::SelectObjectContentEventStreamError;
    fn unmarshall(
        &self,
        message: &aws_smithy_eventstream::frame::Message,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        aws_smithy_eventstream::error::Error,
    > {
        let response_headers = aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => match response_headers.smithy_type.as_str() {
                "Records" => {
                    let mut builder = crate::model::records_event::Builder::default();
                    let content_type = response_headers.content_type().unwrap_or_default();
                    if content_type != "application/octet-stream" {
                        return Err(aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                                "expected :content-type to be 'application/octet-stream', but was '{}'",
                                                content_type
                                            )));
                    }
                    builder = builder.set_payload(Some(aws_smithy_types::Blob::new(
                        message.payload().as_ref(),
                    )));
                    Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::model::SelectObjectContentEventStream::Records(builder.build()),
                    ))
                }
                "Stats" => {
                    let mut builder = crate::model::stats_event::Builder::default();
                    builder = builder.set_details(Some(
                        crate::xml_deser::deser_member_com_amazonaws_s3_stats_event_details(
                            &message.payload()[..],
                        )
                        .map_err(|err| {
                            aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                "failed to unmarshall details: {}",
                                err
                            ))
                        })?,
                    ));
                    Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::model::SelectObjectContentEventStream::Stats(builder.build()),
                    ))
                }
                "Progress" => {
                    let mut builder = crate::model::progress_event::Builder::default();
                    builder = builder.set_details(Some(
                        crate::xml_deser::deser_member_com_amazonaws_s3_progress_event_details(
                            &message.payload()[..],
                        )
                        .map_err(|err| {
                            aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                "failed to unmarshall details: {}",
                                err
                            ))
                        })?,
                    ));
                    Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::model::SelectObjectContentEventStream::Progress(builder.build()),
                    ))
                }
                "Cont" => Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::model::SelectObjectContentEventStream::Cont(
                        crate::model::ContinuationEvent::builder().build(),
                    ),
                )),
                "End" => Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::model::SelectObjectContentEventStream::End(
                        crate::model::EndEvent::builder().build(),
                    ),
                )),
                _unknown_variant => Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::model::SelectObjectContentEventStream::Unknown,
                )),
            },
            "exception" => {
                let generic =
                    match crate::xml_deser::parse_event_stream_generic_error(message.payload()) {
                        Ok(generic) => generic,
                        Err(err) => {
                            return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                crate::error::SelectObjectContentEventStreamError::unhandled(err),
                            ))
                        }
                    };
                Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::error::SelectObjectContentEventStreamError::generic(generic),
                ))
            }
            value => {
                return Err(aws_smithy_eventstream::error::Error::unmarshalling(
                    format!("unrecognized :message-type: {}", value),
                ));
            }
        }
    }
}
