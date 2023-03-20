use bindings::{instance_network, network, streams, tcp, tcp_create_socket};

fn main() -> Result<(), ()> {
    // Create a socket, bind it to localhost, listen, and accept a connection.
    let network = instance_network::instance_network();
    let socket = tcp_create_socket::create_tcp_socket(network::IpAddressFamily::Ipv4).unwrap();
    let address = network::IpSocketAddress::Ipv4(network::Ipv4SocketAddress {
        address: (127, 0, 0, 1),
        port: 0,
    });
    tcp::bind(socket, network, address).unwrap();
    tcp::listen(socket, network).unwrap();

    eprintln!("listening on {:?}", tcp::local_address(socket).unwrap());

    let (_tcp_socket, input, output) = tcp::accept(socket).unwrap();

    // Read data from the socket and echo it back.
    loop {
        let (data, eos) = streams::blocking_read(input, u64::MAX).unwrap();
        let mut view = &data[..];
        while !view.is_empty() {
            let n = streams::blocking_write(output, &view).unwrap() as usize;
            view = &view[n..];
        }
        if eos {
            break;
        }
    }

    Ok(())
}
