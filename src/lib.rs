use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[allow(warnings)]
mod bindings;
use bindings::wasi::sockets0_2_0::tcp_create_socket::{create_tcp_socket, IpAddressFamily, TcpSocket};
use bindings::wasi::sockets0_2_0::network::{IpSocketAddress, Ipv4SocketAddress, Ipv4Address};
use bindings::wasi::sockets0_2_0::instance_network::instance_network;


/// A simple Spin HTTP component.
#[http_component]
fn handle_spin_practice(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let ss:TcpSocket = create_tcp_socket(IpAddressFamily::Ipv4).unwrap();
    let nn = instance_network();
    let ipv4_address:Ipv4Address = (127, 0, 0, 1);
    let ipv4_socket_address = Ipv4SocketAddress { port: 8080, address: ipv4_address };
    let c = ss.start_connect(&nn, IpSocketAddress::Ipv4(ipv4_socket_address));

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())

}
