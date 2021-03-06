use futures::Future;
use libp2p_secio::{SecioConfig, SecioOutput};
use libp2p_core::{Multiaddr, identity, upgrade::apply_inbound};
use libp2p_core::transport::Transport;
use libp2p_tcp::TcpConfig;
use tokio_io::io::write_all;
use tokio::runtime::current_thread::Runtime;

let dialer = TcpConfig::new()
    .with_upgrade({
        // See the documentation of `identity::Keypair`.
	    let keypair = libp2p::identity::Keypair::generate_ed25519();

        SecioConfig::new(keypair)
    })
    .map(|out: SecioOutput<_>, _| out.stream);

let future = dialer.dial("/ip4/127.0.0.1/tcp/12345".parse::<Multiaddr>().unwrap())
    .unwrap()
    .map_err(|e| panic!("error: {:?}", e))
    .and_then(|connection| {
        // Sends "hello world" on the connection, will be encrypted.
        write_all(connection, "hello world")
    })
    .map_err(|e| panic!("error: {:?}", e));

let mut rt = Runtime::new().unwrap();
let _ = rt.block_on(future).unwrap();