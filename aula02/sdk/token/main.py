from wallet import create_alice_wallet, create_bob_wallet
from utils import create_account, get_faucet
from tokens import create_token, balance_of_token
from stellar_sdk import Server

print("✅ # Start creating a custom token on Stellar")

print("✅ # Configure the server for the Standalone network (local)")
server = Server("http://localhost:8000")

# Criar carteiras para emissor e distribuidor
issuer = create_alice_wallet()
print(f"✅ # Issuer public key: {issuer.public_key}")

distributor = create_bob_wallet()
print(f"✅ # Distributor public key: {distributor.public_key}")

# Depositar fundos na conta do emissor e do distribuidor
get_faucet(issuer.public_key, server)
# get_faucet(distributor.public_key, server)

# Criar conta no Ledger para distribuidor
starting_balance = 1000
create_account(issuer, distributor.public_key, starting_balance, server)

# Criar token personalizado
ASSET_CODE = "NEARX"
AMOUNT = 1_000_000

token = create_token(issuer, distributor, ASSET_CODE, AMOUNT, server)


balance_of_token(issuer, server)
balance_of_token(distributor, server)


print("✅ # Done!")
