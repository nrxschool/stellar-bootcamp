# Soroban 102

## make alias

```bash
# ~/.zshrc
# ~/.bashrc

alias ska="soroban keys address "
alias skl="soroban keys ls "
alias scd="soroban contract deploy "
alias sci="soroban contract invoke "
```

## Deploy contract

```bash
scd \
  --wasm target/wasm32-unknown-unknown/release/token.wasm \
  --source bob \
  --network local
```

## Get balance

```bash
sci \
    --id CCIFKHKLPDAIITTUNSNLBKE4EIWLSR4JY2YTIJXACJWXPBEMUKTU6WTP \
    --source bob \
    --network local \
    -- \
    balance \
    --id $(ska bob)
```

## Initialization of contract

```bash
sci \
    --id CCIFKHKLPDAIITTUNSNLBKE4EIWLSR4JY2YTIJXACJWXPBEMUKTU6WTP \
    --source bob \
    --network local \
    -- \
    initialize --admin $(ska bob) --decimal 2 --name RealDigital --symbol DREX
```

## Get admin of contract

```bash
sci \
    --id CCIFKHKLPDAIITTUNSNLBKE4EIWLSR4JY2YTIJXACJWXPBEMUKTU6WTP \
    --source bob \
    --network local \
    -- \
    get_admin
```

## Mint _DREX_ for `alice`

```bash
sci \
    --id CCIFKHKLPDAIITTUNSNLBKE4EIWLSR4JY2YTIJXACJWXPBEMUKTU6WTP \
    --source bob \
    --network local \
    -- \
    mint --to $(ska alice) --amount 100
```

## Transfer from `alice` to `bob`

```bash
sci \
    --id CCIFKHKLPDAIITTUNSNLBKE4EIWLSR4JY2YTIJXACJWXPBEMUKTU6WTP \
    --source alice \
    --network local \
    -- \
    transfer \
    --from $(ska alice) \
    --to $(ska bob) \
    --amount 100
```

## Approve from `alice` to `bob`

```bash
sci \
    --id CCIFKHKLPDAIITTUNSNLBKE4EIWLSR4JY2YTIJXACJWXPBEMUKTU6WTP \
    --source alice \
    --network local \
    -- \
    approve \
    --from $(ska alice) \
    --to $(ska bob) \
    --amount 100
```

## TransferFrom `bob` spend `alice`'s money

```bash
sci \
    --id CCIFKHKLPDAIITTUNSNLBKE4EIWLSR4JY2YTIJXACJWXPBEMUKTU6WTP \
    --source bob \
    --network local \
    -- \
    trasfer_from \
    --from $(ska alice) \
    --to $(ska bob) \
    --amount 100
```
