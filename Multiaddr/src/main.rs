use libp2p::core::multiaddr::{
    *,
    Protocol
};

use std::net::Ipv4Addr;


fn main() {
    // push
    let mut address: Multiaddr = "/ip4/127.0.0.1".parse().unwrap();
    address.push(Protocol::Tcp(10000));

    // empty
    let emptyaddr : Multiaddr = Multiaddr::empty();

    // with_capacity
    let capacity : Multiaddr = Multiaddr::with_capacity(3);

    println!("{},{},{}",address, emptyaddr, capacity);

    // pop
    let popit = address.pop().unwrap();
    println!("{}",popit);

    // Tcp
    let new = Protocol::Tcp(10000);
    println!("{}", new);

    // pop
    address = "/ip4/127.0.0.1/udt/sctp/5678".parse().unwrap();
    println!("{}", address.pop().unwrap());
    println!("{}", address.pop().unwrap());

    // iter
    address = "/ip4/127.0.0.1/udt/sctp/5678".parse().unwrap();
    let components = address.iter().collect::<Vec<_>>();
    assert_eq!(components[0], Protocol::Ip4(Ipv4Addr::new(127, 0, 0, 1)));
    println!("{}, {}, {}",components[0], components[1], components[2]);

    //replace
//    address = address.replace(2,Protocol::Sctp(56700)).unwrap();
    println!("{}", address);
}
