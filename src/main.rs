use crate::server::Server;
use crate::session::Session;

use actix::{Actor, Addr};
use actix_web::{
    get,
    middleware::Logger,
    web::{Data, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws::start;

mod server;
mod session;

#[get("/")]
async fn websocket(
    req: HttpRequest,
    stream: Payload,
    server: Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    start(Session::new(server.get_ref().clone()), &req, stream)
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new().start();
    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .service(websocket)
            .wrap(Logger::default())
    })
    .bind(("localhost", 8000))?
    .workers(2)
    .run()
    .await
}
