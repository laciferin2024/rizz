import base58
import json

# Replace this with your actual base58 private key string
private_key_base58 = input("Enter your private key  ")

# Convert the base58 string to a byte array
private_key_bytes = base58.b58decode(private_key_base58)

# Convert the byte array to a list of integers
private_key_list = list(private_key_bytes)

# Save the list to a JSON file
with open('my-keypair.json', 'w') as f:
    json.dump(private_key_list, f)

print("Private key has been saved to my-keypair.json")
