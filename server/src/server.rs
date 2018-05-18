

// pub trait Handler {
//     fn on_shutdown(&mut self) { ... }
//     fn on_open(&mut self, shake: Handshake) -> Result<()> { ... }
//     fn on_message(&mut self, msg: Message) -> Result<()> { ... }
//     fn on_close(&mut self, code: CloseCode, reason: &str) { ... }
//     fn on_error(&mut self, err: Error) { ... }
//     fn on_request(&mut self, req: &Request) -> Result<Response> { ... }
//     fn on_response(&mut self, res: &Response) -> Result<()> { ... }
//     fn on_timeout(&mut self, event: Token) -> Result<()> { ... }
//     fn on_new_timeout(&mut self, _: Token, _: Timeout) -> Result<()> { ... }
//     fn on_frame(&mut self, frame: Frame) -> Result<Option<Frame>> { ... }
//     fn on_send_frame(&mut self, frame: Frame) -> Result<Option<Frame>> { ... }
//     fn build_request(&mut self, url: &Url) -> Result<Request> { ... }
//     fn upgrade_ssl_client(
//         &mut self, 
//         stream: TcpStream, 
//         url: &Url
//     ) -> Result<SslStream<TcpStream>> { ... }
//     fn upgrade_ssl_server(
//         &mut self, 
//         _: TcpStream
//     ) -> Result<SslStream<TcpStream>> { ... }
// }

use std::sync::{ Arc, Mutex };
use gamestate::{GameState};
use std::collections::HashMap;
use connection::{Connection};
use ws;

pub struct Server {
    state : Arc<Mutex<GameState>>,
    connections : HashMap<u64, ws::Sender>,
}

impl Server {
    pub fn new(state : Arc<Mutex<GameState>>) -> Server {
        let connections = HashMap::new();
        Server { state, connections }
    }
}

impl ws::Factory for Server {
    type Handler = Connection;
    fn client_connected(&mut self, out: ws::Sender) -> Connection {
        let arc_state = Arc::clone(&self.state);
        let con = Connection::new(arc_state);
        let id = con.get_id();
        self.connections.insert(id, out);
        con
    }

    fn connection_made(&mut self, _ws: ws::Sender) -> Connection {
        panic!("no tahanks!")
    }
}

pub fn listen(host : &str, port : u32, state : &Arc<Mutex<GameState>> ) -> ws::Result<ws::WebSocket<Server>> {
    use std::net::{ToSocketAddrs};

    let state = Arc::clone(state);
    let server = Server::new(state);

    info!("Starting a simpleserver on port {}", port);

    let con_str = format!("{}:{}", host, port);

    let mut addr = con_str.to_socket_addrs()?;

    let first_addr = addr.next().unwrap();
    
    let ws = ws::WebSocket::new(server)?;

    ws::WebSocket::listen(ws,first_addr)
}
