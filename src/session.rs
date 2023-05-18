use crate::server::Server;

use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, Addr, AsyncContext, StreamHandler};
use actix_web::web::Bytes;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};

static HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
static HEARTBEAT_LIMIT: Duration = Duration::from_secs(10);

pub struct Session {
    server: Addr<Server>,
    user_id: Option<String>,
    heartbeat: Instant,
}

impl Session {
    pub fn new(server: Addr<Server>) -> Self {
        Self {
            server,
            user_id: None,
            heartbeat: Instant::now(),
        }
    }

    fn register(&mut self, user_id: String) {
        self.user_id = Some(user_id);
    }

    fn hb(&mut self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            let last_beat = Instant::now() - act.heartbeat;
            if last_beat > HEARTBEAT_LIMIT {
                log::info!("Client timed out!");
                ctx.stop();
            }
        });
    }

    fn handle_message(&mut self, message: String) {}

    fn handle_binary_message(&mut self, bytes: &Bytes) {}
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for Session {
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(Message::Ping(bytes)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&bytes)
            }
            Ok(Message::Pong(_)) => self.heartbeat = Instant::now(),
            Ok(Message::Text(text)) => {
                self.heartbeat = Instant::now();
                self.handle_message(text.to_string());
            }
            Ok(Message::Binary(bin)) => {
                self.heartbeat = Instant::now();
                self.handle_binary_message(&bin);
            }
            Ok(Message::Close(reason)) => ctx.close(reason),
            Ok(_) => {
                self.heartbeat = Instant::now();
            }
            Err(err) => {
                log::error!("{err}");
                ctx.stop();
            }
        }
    }
}
