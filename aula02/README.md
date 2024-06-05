# 0. Blockchain paths

- core blockchain engineer -> node
  * redes, criptografia, protocolo de consenso, DevOps

- SDK engineer -> SDK
  * python, javascript, rust, ruby, golang

- smartcontract engineer -> smartcontract
  * EVM: solidity, vyper | no-EVM: Rust, Golang

- integration web3 engine -> application
  * lib (SDK): web3js, etherjs, viem  | cardano SDK 

# 1. Como funciona o Ecosistema Stellar?

- Nodes (Consenso)
- SDK (Horizon)
- Soroban (Smartcontacts)
- Wallet (Freighter)

# 2. Instalar ferramentas

- Docker
- Python


# 3. Configurarr um node para desenvolvimento

**Deploy do Node via Docker**

```
docker run \
  --rm \
  -p 8000:8000 \
  --name stellar \
  stellar/quickstart:testing \
  --local \
  --enable-soroban-rpc
```

**Deploy do [Node + Prometheus + Grafana] via Docker Compose**

- [docker-compose.yaml](./setup-node/docker-compose.yaml)

# 4. Usando o SDK para criar contas e fazer um pagamento

- Como criar uma conta
- Como criar uma transação

# 5. Usando SDK para criar um Token

- Emissor e Distribuidor
- Criando o Token
- Destribuindo o Token

# 6. Criando um Pool de Liquidez

- Tipos de transações

# 7. Executando ordens de compra e venda

- Colocando uma ordem de venda
- Colocando uma ordem de compra
- Validando saldos nas carteiras
