# Como funciona o Ecosistema Stellar?

- Nodes (consenso)
- RPC
- Wallet
- Tx

# Instalando ferramentas

- Rust
- Docker
- Soroban

# Como subir um node para desenvolvimento

Deploy do Node

```
docker run \
  --rm \
  -p 8000:8000 \
  --name stellar \
  stellar/quickstart:testing \
  --standalone \
  --enable-soroban-rpc
```

Configurar o client da rede

```
soroban config network add standalone \
    --rpc-url "http://localhost:8000/soroban/rpc" \
    --network-passphrase "Standalone Network ; February 2017"
```

Configurar o client da wallet

```
soroban keys generate --global bob --network standalone
```

# Como interagir com o RPC

Consultar saldo

```

```

Consultar Tx

```

```

Consultar Bloco

```

```

Consultar um Smartcontract

```

```

# Como fazer deploy de Smartcontract

fazer build

```
soroban contract build
```

fazer o deploy

```
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source bob \
  --network standalone
```

# Como interagir com o Smartcontract

```
soroban contract invoke \
 --id CDSX6IYWTVZI7QUBNFFHETHEQA3TB5M3EW6RI66ZZODP74XZYBUV7YY4 \
 --source alice \
 --network testnet \
 -- \
 hello \
 --arg Lucas
```


According to the Stellar documentation, Stellar uses the following cryptographic algorithms for key generation and signing:

Key Generation
Stellar uses the ed25519 elliptic curve digital signature algorithm for generating key pairs (public and private keys). This is an implementation of the EdDSA digital signature scheme, which is based on Curve25519.

The key generation process is as follows:
A random 256-bit seed is generated
This seed is hashed using the SHA-512 hash function to produce a 512-bit digest
The first 256 bits of the digest become the private key
The public key is derived from the private key using the ed25519 algorithm

Signing
Stellar uses the ed25519 signature scheme for signing transactions and other data. The private key is used to create a digital signature over the data being signed.

Hashing
For hashing data like transactions, Stellar uses the SHA-256 and RIPEMD-160 hash functions, specifically the "RIPEMD160(SHA256(data))" construction.

In summary:
Key Generation: ed25519 (based on Curve25519)
Signing: ed25519
Hashing: SHA-256 and RIPEMD-160%
