from utils import get_faucet, get_ledger_by_sequence, get_transaction_by_hash
from wallet import create_alice_wallet, create_bob_wallet
from transfer import transfer_from
from stellar_sdk import Server


print("✅ # Start Payment from Alice to Bob with amount 100XML")


print("✅ # Configure the server for the Standalone network (local)")
server = Server(horizon_url="http://localhost:8000")


# criar carteiras
alice = create_alice_wallet()
bob = create_bob_wallet()


# depositar nas carteiras
get_faucet(alice.public_key, server)
get_faucet(bob.public_key, server)


# fazer transferencia
tx_hash = transfer_from(alice, bob.public_key, 1000, server)


# Buscar transação
transaction = get_transaction_by_hash(server, tx_hash)


# Buscar bloco da transação
ledger_sequence = transaction["ledger"]
ledger = get_ledger_by_sequence(server, ledger_sequence)

