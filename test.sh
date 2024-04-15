#!/bin/bash

# Generate a random key using md5sum of the current timestamp
random_key=$(openssl rand -hex 12)
value="testvalue"

# Set a value
echo "Setting value for key $random_key..."
response=$(curl -s -X POST -d "$value" "http://localhost:9876/set/mycollection/$random_key")
echo "$response"
if [[ "$response" != "Value set successfully" ]]; then
    echo "Failed to set value. Test aborted."
    exit 1
fi

# Get the value
echo -e "\nGetting value for key $random_key..."
response=$(curl -s "http://localhost:9876/get/mycollection/$random_key")
echo "Response: $response"
if [[ "$response" != "$value" ]]; then
    echo "Error: Retrieved value does not match set value."
    exit 1
fi

# Unset the value
echo -e "\nUnsetting value for key $random_key..."
response=$(curl -s -X DELETE "http://localhost:9876/unset/mycollection/$random_key")
echo "$response"
if [[ "$response" != "Removed: $value" ]]; then
    echo "Failed to unset key."
    exit 1
fi

# Try to get the value again
echo -e "\nTrying to get the unset value for key $random_key..."
response=$(curl -s "http://localhost:9876/get/mycollection/$random_key")
echo "Response: $response"
if [[ "$response" != "Key not found" ]]; then
    echo "Error: Expected no value but got '$response'."
    exit 1
fi

echo -e "\nAll tests passed successfully."
