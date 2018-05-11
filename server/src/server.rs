

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


