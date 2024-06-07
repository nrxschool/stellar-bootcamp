# Soroban 101

## 1. Flipper

## 2. Counter

## 3. Task Manager



## 1. config networking

```
soroban config network add local \
    --rpc-url "http://localhost:8000/soroban/rpc" \
    --network-passphrase "Standalone Network ; February 2017"
```

## 2. config wallet

```
soroban keys generate --global bob --network local
```

## 3. build smartcontracts

```
soroban contract build
```

## 4. deploy smartcontract

```
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/<CONTRACT_NAME>.wasm \
  --source bob \
  --network local
```

## 5. interact with smartcontract

```
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source bob \
  --network local \
  -- \
  hello \
  --to Lucas
```

```bash
soroban contract invoke \
  --id CDGRZ4K2YZAP2LWU5XTCAQM3AF5MD4QWY6KZO274OJ7PSFW6BJIRQPDV \
  --source bob \
  --network local \
  -- \
  get_all_tasks
```

```bash
soroban contract invoke \
  --id CDGRZ4K2YZAP2LWU5XTCAQM3AF5MD4QWY6KZO274OJ7PSFW6BJIRQPDV \
  --source bob \
  --network local \
  -- \
  add_task \
  --name Limpar_o_carro
```

```bash
soroban contract invoke \
  --id CDGRZ4K2YZAP2LWU5XTCAQM3AF5MD4QWY6KZO274OJ7PSFW6BJIRQPDV \
  --source bob \
  --network local \
  -- \
  complete_task \
  --id 1
```

```bash
soroban contract invoke \
  --id CDGRZ4K2YZAP2LWU5XTCAQM3AF5MD4QWY6KZO274OJ7PSFW6BJIRQPDV \
  --source bob \
  --network local \
  -- \
  delete_task \
  --id 12
```