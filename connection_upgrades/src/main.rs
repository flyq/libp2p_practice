use libp2p::{Transport, tcp::TcpConfig, secio::SecioConfig, identity::Keypair};


fn main() {
let tcp = TcpConfig::new();
let secio_upgrade = SecioConfig::new(Keypair::generate_ed25519());
let tcp_secio = tcp.with_upgrade(secio_upgrade);



}
