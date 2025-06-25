use short_link::startup::start;
use std::net::TcpListener;

pub fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
  let port = listener.local_addr().unwrap().port();
  let server = start(listener).expect("Failed to create server");
  tokio::spawn(server);
  format!("http://127.0.0.1:{}", port)
}
