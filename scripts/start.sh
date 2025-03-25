#!/bin/bash

# path of config.yaml
config_file=$1

if [ -z "$config_file" ]; then
  echo "config_file is not set. Exiting..."
  exit 1
fi

if [ ! -f "$config_file" ]; then
  echo "config file not existed. Exiting..."
  exit 1
fi

echo "Initializing the database: ${config_file}"
./target/release/operator-runer -i $config_file

echo "Starting the application..."
./target/release/operator-runer -c $config_file &

echo "Application started successfully."
exec tail -f /dev/null
