extern crate hyper;
extern crate websocket;

mod http_server;
mod websocket_server;

fn main() {
	// Start listening for http connections
	http_server::start();

	// Start listening for WebSocket connections
	websocket_server::start();
}
