use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:8333")?;
        let dst_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8303);

        let mut downstream_client: Option<SocketAddr> = None;
        let mut buf = [0; 2048];

        loop {
            let (len, src) = socket.recv_from(&mut buf)?;
            let buf = &mut buf[..len];

            if src.to_string() == dst_addr.to_string() {
                println!("[from_upstream_server] addr={} data={:?}", src.to_string(), buf);
                if let Some(addr) = downstream_client {
                    socket.send_to(buf, addr)?;
                }
            } else {
                if downstream_client == None {
                    println!("[info] client connected with addr={}", src);
                }
                downstream_client = Some(src);

                println!("[from_downstream_client] addr={} data={:?}", src.to_string(), buf);
                socket.send_to(buf, dst_addr)?;
            }

            // socket.send_to(buf, "127.0.0.1:8303")?;
        }
    }
}

// how do you call the two sides of a proxy? xd
//
//
//
// client [fake_server_socket] <---- [proxy] ----> [fake_client_socket] server
// downstream?                                                          upstream?
//
