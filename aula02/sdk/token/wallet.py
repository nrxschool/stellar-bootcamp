from stellar_sdk import Keypair


def create_alice_wallet():
    print("✅ # Alice's wallet created (ISSUER)")
    alice_secret_key = "SAKKTW5AEJO7Y5DOGUKINE6NF55CR37OWK3WIX4JWQ7LHOOUIH2VQYR4"
    alice = Keypair.from_secret(alice_secret_key)
    assert alice.public_key == "GBTIDN5UQ3P4HT3CSIEYU55F6INXOMYOTU6U5KM67QMXPPUCTABT35U5"

    return alice


def create_bob_wallet():
    print("✅ # Bob's wallet created (DISTRIBUTOR)")
    bob_secret_key = "SCHBR3CYVBCN4DKDML5VKMAP3Q7B7XN6ARI34UHCOEBHLMNXEUTGIXI5"
    bob = Keypair.from_secret(bob_secret_key)
    assert bob.public_key == "GCGRVQLR2BKLNP3XP3N2R47Z52X2OIBC7QVZUI4LAZYXRRVIN5TP5GVJ"

    return bob
