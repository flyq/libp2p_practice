use libp2p::{
	Multiaddr,
	Transport,
	tcp::TcpConfig
    };

fn main() {
    let tcp = TcpConfig::new();
    let addr: Multiaddr = "/ip4/98.97.96.95/tcp/20500".parse().expect("invalid multiaddr");
    let _conn = tcp.dial(addr);

    let keypair = libp2p::identity::Keypair::generate_ed25519();
    let _transport = libp2p::build_development_transport(keypair);

}
