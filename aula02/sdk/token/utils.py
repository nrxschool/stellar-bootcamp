from stellar_sdk import Asset, Network, TransactionBuilder
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


def create_account(funder_wallet, new_account_public_key, starting_balance, server):
    funder_account = server.load_account(funder_wallet.public_key)

    print("✅ # Set the base fee")
    try:
        fee = server.fetch_base_fee()
    except Exception as e:
        print(f"Failed to fetch base fee: {e}")
        fee = 100

    print("✅ # Build transaction")
    transaction = (
        TransactionBuilder(
            source_account=funder_account,
            network_passphrase=Network.STANDALONE_NETWORK_PASSPHRASE,
            base_fee=fee,
        )
        .append_create_account_op(
            destination=new_account_public_key, starting_balance=str(starting_balance)
        )
        .append_payment_op(new_account_public_key, Asset.native(), "10.25")
        .set_timeout(30)
        .build()
    )

    print("✅ # Sign the transaction with Sender's secret key")
    transaction.sign(funder_wallet.secret)

    print("✅ # Submits the transaction to the Horizon server")
    response = server.submit_transaction(transaction)

    tx_hash = response["hash"]
    print(f"✅ # Transaction Hash: {tx_hash}")

    if response["successful"]:
        print("✅ # Create account successful")
    else:
        raise Exception("Transaction was failed to be created")

    return response

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