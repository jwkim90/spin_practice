#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http0_2_0::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};
use bindings::wasi::http0_2_0::types::{Fields, OutgoingBody, OutgoingResponse};

use bindings::wasi::sockets0_2_0::tcp_create_socket::{create_tcp_socket, IpAddressFamily, TcpSocket};
use bindings::wasi::sockets0_2_0::network::{IpSocketAddress, Ipv4SocketAddress, Ipv4Address, IpAddress, Network};
use bindings::wasi::sockets0_2_0::instance_network::instance_network;


struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        println!("Handling request to {:?}://{:?}/{:?}", request.scheme(), request.authority(), request.path_with_query());


        let ss:TcpSocket = match create_tcp_socket(IpAddressFamily::Ipv4) {
            Ok(socket) => socket,
            Err(e) => {
                println!("Failed to create TCP socket: {:?}", e);
                return;
            }
        };

        let nn = instance_network();


        let ipv4_address:Ipv4Address = (127, 0, 0, 1);
        let ipv4_socket_address = Ipv4SocketAddress { port: 9999, address: ipv4_address };

        match ss.start_connect(&nn, IpSocketAddress::Ipv4(ipv4_socket_address)) {
            Ok(_) => {
                println!("Connection started successfully.");
                // Further code to use the socket...
            }
            Err(e) => {
                println!("Failed to start connection: {:?}", e);
                // Log detailed error information
                println!("Error Name: {}", e.name());
                println!("Error Message: {}", e.message());
            }
        }


        let response = OutgoingResponse::new(Fields::new());
        let body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));

        let resp_stm = body.write().unwrap();
        resp_stm.write("Hello Fermyon!".as_bytes()).unwrap();
        resp_stm.flush().unwrap();
        OutgoingBody::finish(body, None).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
