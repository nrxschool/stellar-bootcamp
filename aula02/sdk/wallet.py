from stellar_sdk import Keypair


def create_alice_wallet():
    print("✅ # Alice's wallet created (SENDER)")
    alice_secret_key = "SAKKTW5AEJO7Y5DOGUKINE6NF55CR37OWK3WIX4JWQ7LHOOUIH2VQYR4"

    return Keypair.from_secret(alice_secret_key)


def create_bob_wallet():
    print("✅ # Bob's wallet created (RECEIVER)")
    bob_secret_key = "SCHBR3CYVBCN4DKDML5VKMAP3Q7B7XN6ARI34UHCOEBHLMNXEUTGIXI5"

    return Keypair.from_secret(bob_secret_key)
