#[cfg(feature = "enable-streaming")]
pub(crate) mod audio_stream {
    use actix_web::{HttpRequest, HttpResponse, rt};
    use actix_web::web::Payload;
    use actix_ws::Message;
    use futures_util::{future, StreamExt};
    use std::time::{Duration, Instant};
    use tokio::pin;
    use tokio::time::interval;

    pub async fn ws_streaming(
        req: HttpRequest,
        stream: Payload,
    ) -> Result<HttpResponse, actix_web::Error> {
        let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;
        rt::spawn(parse_msg(session, msg_stream));
        Ok(response)
    }

    async fn parse_msg(mut session: actix_ws::Session, mut msg_stream: actix_ws::MessageStream) {
        log::info!("connected");

        let mut last_heartbeat = Instant::now();
        let mut interval = interval(Duration::from_secs(5));

        let reason = loop {
            let tick = interval.tick();
            // required for select()
            pin!(tick);

            match future::select(msg_stream.next(), tick).await {
                future::Either::Left((Some(Ok(msg)), _)) => {
                    log::debug!("msg: {msg:?}");

                    match msg {
                        Message::Text(text) => {
                            session.text(text).await.unwrap();
                        }

                        Message::Binary(bin) => {
                            session.binary(bin).await.unwrap();
                        }

                        Message::Close(reason) => {
                            break reason;
                        }

                        Message::Ping(bytes) => {
                            last_heartbeat = Instant::now();
                            let _ = session.pong(&bytes).await;
                        }

                        Message::Pong(_) => {
                            last_heartbeat = Instant::now();
                        }

                        Message::Continuation(_) => {
                            log::warn!("no support for continuation frames");
                        }

                        // no-op; ignore
                        Message::Nop => {}
                    };
                }
                future::Either::Left((Some(Err(err)), _)) => {
                    log::error!("{}", err);
                    break None;
                }

                // client WebSocket stream ended
                future::Either::Left((None, _)) => break None,

                future::Either::Right((_inst, _)) => {
                    // if no heartbeat ping/pong received recently, close the connection
                    if Instant::now().duration_since(last_heartbeat) > Duration::from_secs(10) {
                        let ct = Duration::from_secs(10);
                        log::info!("client has not sent heartbeat in over {ct:?}; disconnecting");

                        break None;
                    }

                    // send heartbeat ping
                    let _ = session.ping(b"").await;
                }
            }
        };

        let _ = session.close(reason).await;

        log::info!("disconnected");
    }
}
