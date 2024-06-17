use crate::whisper::forms::RecognizeParameters;
use crate::whisper::client::WhisperClient;
use crate::whisper::config::WhisperClientConfig;

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebsocketActor {
    hb: Instant,
    client: WhisperClient,
}

impl WebsocketActor {
    pub fn new(wh_cfg: &WhisperClientConfig) -> Self {
        Self {
            hb: Instant::now(),
            client: WhisperClient::new(wh_cfg),
        }
    }

    // This function will run on an interval, every 5 seconds to check
    // that the connection is still alive. If it's been more than
    // 10 seconds since the last ping, we'll close the connection.
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebsocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketActor {
    // The `handle()` function is where we'll determine the response
    // to the client's messages. So, for example, if we ping the client,
    // it should respond with a pong. These two messages are necessary
    // for the `hb()` function to maintain the connection status.
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            // Text will echo any text received back to the client (for now)
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(data)) => {
                let data_vec = data.to_vec();
                let params = RecognizeParameters::default();
                match self.client.recognize_chunk(data_vec.as_slice(), &params) {
                    Err(err) => {
                        ctx.text(err.to_string())
                    }
                    Ok(resp) => {
                        let values = serde_json::to_value(resp).unwrap();
                        ctx.text(values.to_string())
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
