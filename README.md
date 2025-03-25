# AOS Operator

## Overview

The AOS Operator is a role of EigenAVS in Hetu Protocols. 

By registering with AOS on Dispatcher, the operator could service the AI inference verification task.The staker can delegate funds to an operator by Delegation Manager contract.

## Compile and Run

1. **Install `cargo` Tool:**

   The Rust toolchain can be installed via Rustup. Execute the following command:

   ```bash
   curl https://sh.rustup.rs -sSf | sh
   ```

   After installation, ensure that the Rust toolchain path is added to your environment variable:

   ```bash
   source $HOME/.cargo/env
   ```

2. **Install Dependencies:**

   On Ubuntu systems, run the following command to install dependencies:

   ```bash
   sudo apt-get update && sudo apt-get install \
   curl \
   libssl-dev \
   libpq-dev \
   openssl \
   ```
   
   On Fedora systems, run the following commands:

   ```bash
   sudo yum groupinstall 'Development Tools'
   sudo yum install openssl-devel postgresql-libs postgresql-devel
   ```

3. **Install PostgreSQL:**

   - On Ubuntu systems:

     ```bash
     sudo apt install postgresql
     ```

   - On Fedora systems:

     ```bash
     sudo yum install postgresql-server
     sudo yum install postgresql16.x86_64 postgresql16-server -y
     sudo postgresql-setup --initdb
     ```

   **Start PostgreSQL Service:**

   ```bash
   sudo systemctl start postgresql
   sudo systemctl enable postgresql
   ```

4. **Install Redis:**

   - On Ubuntu systems:

     ```bash
     sudo apt-get install redis
     ```

   - On Fedora systems:

     ```bash
     sudo yum install -y redis6
     ```

   **Start Redis Service:**

   ```bash
   sudo systemctl enable redis-server
   sudo systemctl start redis-server
   ```

5. **Compile the Source Code:**

   Use the following command to build the project:

   ```bash
   cargo build --release
   ```

6. **Update the Configuration File:**

   Configuration file location: `docs/template/config-operator.yaml`

   Update the following configuration items based on your environment:
   ```yaml
   pg_db_url: PostgreSQL database URL
   dispatcher_url: AOS dispatcher URL
   dispatcher_address: AOS dispatcher ID
   node_id: Operator address
   signer_key: Operator private key
   vrf_key: vrf sign key
   chain_rpc_url: Ethereum RPC node
   ```

7. **Initialize the Database:**

   Use the following command to initialize the PostgreSQL database. Replace 'postgres:hetu' 
   in the URL below with the actual username and password, and replace 'operator_db' with 
   the actual database name.
   
   ```bash
   ./target/debug/operator-runer -i postgres://postgres:hetu@0.0.0.0:5432/operator_db
   ```

8. **Run the Operator:**

   Use the following command to start the operator service:

   ```bash
   ./target/release/operator-runer -c ./docs/template/config-operator.yaml
   ```

## Docker Deployment
1. **Update the docker-compose.yml:**

   Update the following configuration items based on your environment:
   ```yaml
   WORKER_URL: ""         # worker url
   DISPATCHER_URL: ""     # dispatcher url
   DISPATCHER_ADDRESS: "" # dispatcher address
   CHAIN_RPC_URL: ""      # Ethereum RPC node
   VRF_RANGE_CONTRACT: "" # vrf range contract address
   NODE_ID: ""            # operator address
   NODE_TYPE: "opml"      # opml or tee
   SIGNER_KEY: ""         # operator remote signer server
   VRF_KEY: ""            # vrf sign key
   ```

5. **Run the Docker Compose:**

   Use the following command to run the Docker container:

   ```bash
   docker compose up -d
   ```

   Replace `/path/to/config-operator.yaml` with the actual path to the configuration file.

