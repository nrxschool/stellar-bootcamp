# Soroban 102

## 1. Criando um Token

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm \
  --source bob \
  --network standalone
>>> CA3IAQCMJOJJQXEJXHZPXCBNYQ7JI5FA4XRXL4L4OPOZIUEWFFWZNIMM
```

```
soroban contract invoke \
    --id CA3IAQCMJOJJQXEJXHZPXCBNYQ7JI5FA4XRXL4L4OPOZIUEWFFWZNIMM \
    --source bob \
    --network standalone
```

## 2. Criando um Dapp para visualizar
