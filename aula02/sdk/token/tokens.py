from stellar_sdk import Asset, Network, TransactionBuilder
from utils import get_transaction_by_hash


def create_trust_line(issuer_wallet, distributor_wallet, asset_code, amount, server):
    print("✅ # Set the base fee")
    try:
        fee = server.fetch_base_fee()
    except Exception as e:
        print(f"Failed to fetch base fee: {e}")
        fee = 100

    token = Asset(asset_code, issuer_wallet.public_key)
    distributor_account = server.load_account(distributor_wallet.public_key)

    print("✅ # Build transaction")
    transaction = (
        TransactionBuilder(
            source_account=distributor_account,
            network_passphrase=Network.STANDALONE_NETWORK_PASSPHRASE,
            base_fee=fee,
        )
        .add_text_memo("Create a trustline")
        .append_change_trust_op(token, amount)
        .set_timeout(30)
        .build()
    )

    print("✅ # Sign the transaction with Sender's secret key")
    transaction.sign(distributor_wallet.secret)

    print("✅ # Submits the transaction to the Horizon server")
    response = server.submit_transaction(transaction)

    tx_hash = response["hash"]
    print(f"✅ # Transaction Hash: {tx_hash}")

    if response["successful"]:
        print("✅ # Trustline created successful")
    else:
        raise Exception("Transaction was failed")

    get_transaction_by_hash(server, tx_hash)
    return token


def destribution_tokens(issuer_wallet, distributor_wallet, token, amount, server):
    print("✅ # Set the base fee")
    try:
        fee = server.fetch_base_fee()
    except Exception as e:
        print(f"Failed to fetch base fee: {e}")
        fee = 100

    issuer_account = server.load_account(issuer_wallet.public_key)

    print("✅ # Build transaction")
    transaction = (
        TransactionBuilder(
            source_account=issuer_account,
            network_passphrase=Network.STANDALONE_NETWORK_PASSPHRASE,
            base_fee=fee,
        )
        .add_text_memo(f"Token distribution {token.code}")
        .append_payment_op(distributor_wallet.public_key, token, amount)
        .set_timeout(30)
        .build()
    )

    print("✅ # Sign the transaction with Sender's secret key")
    transaction.sign(issuer_wallet.secret)

    print("✅ # Submits the transaction to the Horizon server")
    response = server.submit_transaction(transaction)

    tx_hash = response["hash"]
    print(f"✅ # Transaction Hash: {tx_hash}")

    if response["successful"]:
        print("✅ # Distribuition successful")
    else:
        raise Exception("Transaction was failed")

    get_transaction_by_hash(server, tx_hash)


def create_token(issuer_wallet, distributor_wallet, asset_code, amount, server):

    token = create_trust_line(
        issuer_wallet, distributor_wallet, asset_code, amount, server
    )
    destribution_tokens(issuer_wallet, distributor_wallet, token, amount, server)

    return token


def balance_of_token(wallet, server):
    account = server.accounts().account_id(wallet.public_key).call()
    balances = account["balances"]
    for balance in balances:
        asset_type = (
            balance["asset_code"]
            if balance.get("asset_code", False)
            else balance["asset_type"]
        )
        balance_amount = balance["balance"]
        print(f"✅ # Balances for account {wallet.public_key}:", end=" ")
        print(f"Asset Type: {asset_type}, Balance: {balance_amount}")
