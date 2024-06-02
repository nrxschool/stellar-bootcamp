from utils import get_faucet, get_ledger_by_sequence, get_transaction_by_hash
from wallet import create_alice_wallet, create_bob_wallet
from transfer import transfer_from
from stellar_sdk import Server


print("✅ # Start Payment from Alice to Bob with amount 100XML")


print("✅ # Configure the server for the Standalone network (local)")
server = Server(horizon_url="http://localhost:8000")


# criar carteiras
alice = create_alice_wallet()
assert alice.public_key == "GBTIDN5UQ3P4HT3CSIEYU55F6INXOMYOTU6U5KM67QMXPPUCTABT35U5"

bob = create_bob_wallet()
assert bob.public_key == "GCGRVQLR2BKLNP3XP3N2R47Z52X2OIBC7QVZUI4LAZYXRRVIN5TP5GVJ"


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

1