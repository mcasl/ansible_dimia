#!/bin/bash

# Define the range of IP addresses in your network
network="10.30.48"
filename="up_hosts.txt"

# Function to check if a host is up
check_host() {
    host="$1"
    if ping -c 1 -W 1 "$host" > /dev/null 2>&1; then
        echo "$host" >> "$filename"
    fi
}

# Iterate over each host in the network range and check if it's up
for i in {1..254}; do
    check_host "$network.$i" &
done

# Wait for all background processes to finish
wait

echo "Done. The list of up hosts is saved in $filename"

