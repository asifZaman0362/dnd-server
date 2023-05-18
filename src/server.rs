use crate::session::Session;

use actix::{Actor, Addr, Context, Handler, Message};
use std::collections::HashMap;

pub struct Server {
    rooms: HashMap<String, String>,
    clients: HashMap<String, Addr<Session>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            clients: HashMap::new(),
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Register {
    user_id: String,
    session: Addr<Session>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    user_id: String,
}

impl Handler<Register> for Server {
    type Result = ();
    fn handle(&mut self, msg: Register, _ctx: &mut Self::Context) -> Self::Result {
        if !self.clients.get(&msg.user_id).is_some() {
            self.clients.insert(msg.user_id, msg.session);
        }
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        if self.clients.get(&msg.user_id).is_some() {
            self.clients.remove(&msg.user_id);
        }
    }
}
