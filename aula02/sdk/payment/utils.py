from stellar_sdk.exceptions import NotFoundError
from requests import get, RequestException


def soft_create_account(public_key):
    url = "http://localhost:8000/friendbot"
    params = {"addr": public_key}
    timeout = 30
    try:
        r = get(url, params=params, timeout=timeout)
        r.raise_for_status()
    except RequestException as e:
        raise ValueError(f"Error in get faucet: {str(e)}") from e


def validate_wallet(public_key, server):
    try:
        server.load_account(public_key)
    except NotFoundError:
        print("The destination account does not exist!")
        print("Creating Account!")
        soft_create_account(public_key)


def validate_wallet_balance(public_key, server):
    try:
        r = server.accounts().account_id(public_key).call()
    except Exception as e:
        raise Exception(f"Error in getting account data: {str(e)}") from e

    for balance in r["balances"]:
        if balance["asset_type"] == "native":
            return float(balance["balance"]) > 0


def check_balances(server, public_key):
    account = server.accounts().account_id(public_key).call()
    balances = account["balances"]
    print(f"✅ # Balances for account {public_key}:", end=": ")
    for balance in balances:
        asset_type = balance["asset_type"]
        balance_amount = balance["balance"]
        print(f"Asset Type: {asset_type}, Balance: {balance_amount}")


def get_faucet(public_key, server):
    print(f"✅ # Try deposit 10.000XML to {public_key}")

    validate_wallet(public_key, server)
    if validate_wallet_balance(public_key, server):
        print(f"Wallet: {public_key} already balance!")
        return

    print(f"Wallet: {public_key} need a deposit!")

    soft_create_account(public_key)


def get_transaction_by_hash(server, transaction_hash):
    transaction = server.transactions().transaction(transaction_hash).call()
    print("✅ # Transaction details:")
    print(f"  - ID: {transaction['id']}")
    print(f"  - Hash: {transaction['hash']}")
    print(f"  - Ledger: {transaction['ledger']}")
    print(f"  - Created At: {transaction['created_at']}")
    print(f"  - Source Account: {transaction['source_account']}")
    print(f"  - Memo: {transaction['memo']}")
    print(f"  - Fee Charged: {transaction['fee_charged']}")
    print(f"  - Operation Count: {transaction['operation_count']}")
    print(f"  - Successful: {transaction['successful']}")

    return transaction


def get_ledger_by_sequence(server, sequence):
    ledger = server.ledgers().ledger(sequence).call()
    print("✅ # Ledger details:")
    print(f"  - Sequence: {ledger['sequence']}")
    print(f"  - Hash: {ledger['hash']}")
    print(f"  - Previous Hash: {ledger['prev_hash']}")
    print(f"  - Transaction Count: {ledger['successful_transaction_count']}")
    print(f"  - Operation Count: {ledger['operation_count']}")
    print(f"  - Closed At: {ledger['closed_at']}")
    print(f"  - Total Coins: {ledger['total_coins']}")
    print(f"  - Fee Pool: {ledger['fee_pool']}")
    print(f"  - Base Fee in Stroops: {ledger['base_fee_in_stroops']}")
    print(f"  - Base Reserve in Stroops: {ledger['base_reserve_in_stroops']}")
    print(f"  - Max Transaction Set Size: {ledger['max_tx_set_size']}")
    print(f"  - Protocol Version: {ledger['protocol_version']}")

    return ledger
