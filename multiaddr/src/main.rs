use libp2p::core::multiaddr::{
    *,
    Protocol
};

fn main() {
    let mut address: Multiaddr = "/ip4/127.0.0.1".parse().unwrap();
    address.push(Protocol::Tcp(10000));

    let emptyaddr : Multiaddr = Multiaddr::empty();

    let capacity : Multiaddr = Multiaddr::with_capacity(3);

    println!("{},{},{}",address, emptyaddr, capacity);
}



