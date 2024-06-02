from stellar_sdk import Asset, Network, Server, TransactionBuilder, Keypair
from utils import check_balances


def transfer_from(sender: Keypair, receiver, amount, server: Server):
    print("✅ # Initial Sender's balances")
    check_balances(server, sender.public_key)

    print("✅ # Initial Receiver's balances")
    check_balances(server, receiver)

    print("✅ # Set the base fee")
    try:
        fee = server.fetch_base_fee()
    except Exception as e:
        print(f"Failed to fetch base fee: {e}")
        fee = 100

    print("✅ # Build transaction")
    transaction = (
        TransactionBuilder(
            source_account=server.load_account(sender.public_key),
            network_passphrase=Network.STANDALONE_NETWORK_PASSPHRASE,
            base_fee=fee,
        )
        .add_text_memo(f"Pay {amount}XML to Bob")
        .append_payment_op(receiver, Asset.native(), str(amount))
        .set_timeout(30)
        .build()
    )

    print("✅ # Sign the transaction with Sender's secret key")
    transaction.sign(sender.secret)

    print("✅ # Submits the transaction to the Horizon server")
    response = server.submit_transaction(transaction)
    tx_hash = response["hash"]

    print(f"✅ # Transaction Hash: {tx_hash}")

    print("✅ # Final Sender's balances")
    check_balances(server, sender.public_key)

    print("✅ # Final Receiver's balances")
    check_balances(server, receiver)

    print("✅ # Done!")

    return tx_hash
