#!/bin/bash

CONFIG_FILE="/app/config.yaml"

if [ ! -f "$CONFIG_FILE" ]; then
  cat <<EOF > "$CONFIG_FILE"
db:
  pg_db_url: "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@db:${POSTGRES_PORT}"
  pg_db_name: "${POSTGRES_DB}"
  max_connect_pool: 30
  min_connect_pool: 10
  connect_timeout: 30
  acquire_timeout: 60
queue:
  queue_url: "redis://:@redis:6379"
  topic: "task"
net:
  rest_url: "0.0.0.0:21001"
  callback_url: "http://aos_operator:21001"
  worker_url: "${WORKER_URL}"
  metrics_url: "0.0.0.0:9000"
node:
  node_type: "${NODE_TYPE}"
  node_id: "${NODE_ID}"
  signer_key: "${SIGNER_KEY}"
  vrf_key: "${VRF_KEY}"
  cache_msg_maximum: 500
  heartbeat_interval: 10
chain:
  chain_rpc_url: "${CHAIN_RPC_URL}"
  vrf_range_contract: "${VRF_RANGE_CONTRACT}"
  vrf_sort_precision: 6
api:
  read_maximum: 20
dispatcher:
  dispatcher_url: "${DISPATCHER_URL}"
  dispatcher_address: "${DISPATCHER_ADDRESS}"
EOF
  echo "generate deafult configuration file: $CONFIG_FILE"
fi

if [ ! -f "$CONFIG_FILE" ]; then
  echo "$CONFIG_FILE is not existed, existing..."
  exit 1
fi

echo "finish gengerating configuration file"

bash /app/start.sh $CONFIG_FILE

