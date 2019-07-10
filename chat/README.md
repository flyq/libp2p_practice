# Chat

这个 example 程序主要为了展现 libp2p 的几个 module 的特性。
所以边分析程序边结合标准库文档讲解对应 module 特性

## 开始
从 main 函数入口， env_logger 是设置日志，比如打印 runtime 的日期之类的的。

接下来：
```rust
    // Create a random PeerId
    let local_key = identity::Keypair::generate_ed25519();
    println!("here is local_key.");
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);
```
先从 [libp2p::identity](https://docs.rs/libp2p/0.10.0/libp2p/identity/index.html) 调用生成密钥对的方法。其中可以生成三种类型的密钥对：

**Ed25519(Keypair)**  
* An Ed25519 keypair.

**Rsa(Keypair)**  
* An RSA keypair.

**Secp256k1(Keypair)**
* A Secp256k1 keypair.   
  
这里用到了 ed25519 算法的密钥对。

key 可以通过 public 方法得到改密钥的公钥。 libp2p 库里面的 Structs：PeerId，它的定义是这样：
```rust
pub struct PeerId { /* fields omitted */ }
```
它的方法有很多：
```rust
pub fn from_public_key(key: PublicKey) -> PeerId
// Builds a PeerId from a public key.
  
pub fn from_bytes(data: Vec<u8>) -> Result<PeerId, Vec<u8>>
// Checks whether data is a valid PeerId. If so, returns the PeerId. If not, returns back the data as an error.

pub fn from_multihash(data: Multihash) -> Result<PeerId, Multihash>
// Turns a Multihash into a PeerId. If the multihash doesn't use the correct algorithm, returns back the data as an error.

pub fn random() -> PeerId
// Generates a random peer ID from a cryptographically secure PRNG.

// This is useful for randomly walking on a DHT, or for testing purposes.

pub fn into_bytes(self) -> Vec<u8>
// Returns a raw bytes representation of this PeerId.

// Note that this is not the same as the public key of the peer.

pub fn as_bytes(&self) -> &[u8]
// Returns a raw bytes representation of this PeerId.

// Note that this is not the same as the public key of the peer.

pub fn to_base58(&self) -> String
// Returns a base-58 encoded string of this PeerId.

pub fn digest(&self) -> &[u8]
// Returns the raw bytes of the hash of this PeerId.

pub fn is_public_key(&self, public_key: &PublicKey) -> Option<bool>
// Checks whether the public key passed as parameter matches the public key of this PeerId.

// Returns None if this PeerIds hash algorithm is not supported when encoding the given public key, otherwise Some boolean as the result of an equality check.
```
