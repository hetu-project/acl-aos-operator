use crate::api::msg_handler::{connect_dispatcher, handle_connection, job_result};
use crate::operator::Operator;
use crate::operator::OperatorArc;
use node_api::config;
use node_api::config::OperatorConfig;
use node_api::error::{ErrorCodes, OperatorConfigError};
use std::sync::Arc;
use structopt::StructOpt;
use tracing::*;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics::{counter, gauge};
use sysinfo::System;

use crate::cli::command::{eth_account, init_db};
use std::path::PathBuf;

#[derive(StructOpt)]
struct OperatorCli {
    #[structopt(
        short = "c",
        long = "config",
        parse(from_os_str),
        help = "Yaml file only"
    )]
    config_path: Option<std::path::PathBuf>,

    #[structopt(
        short = "i",
        long = "init_pg",
        help = "Init & refresh pg, caution: new db & new table"
    )]
    init_pg: Option<String>,

    #[structopt(
        short = "k",
        long = "eth_account",
        help = "Gen a eth account, and keypair"
    )]
    eth_account: bool,
}

pub async fn run_cli() {
    let mut help_info = true;
    let args = OperatorCli::from_args();

    // init pg db
    if let Some(pg_conn_str) = args.init_pg {
        help_info = false;
        info!("PostgreSQL connection addr: {}", pg_conn_str);
        // Use the PostgreSQL connection string here for initialization
        if !init_db(pg_conn_str).await {
            return;
        }
    }

    // gen eth account
    if args.eth_account {
        help_info = false;
        info!("Gen a eth account, and keypair");
        // Use the PostgreSQL connection string here for initialization
        if !eth_account() {
            return;
        }
    }

    // setup node
    if let Some(config_path) = args.config_path {
        help_info = false;
        let operator_config = construct_node_config(config_path);

        PrometheusBuilder::new()
        .with_http_listener(operator_config.net.metrics_url.parse::<std::net::SocketAddr>().unwrap())
        .install()
        .expect("failed to install Prometheus exporter");

        info!("Metrics server running on http://{}/metrics", operator_config.net.metrics_url);

        tokio::spawn(cpu_metrics());

        let _operator = build_operator(operator_config.clone()).await;

        let arc_operator_clone = Arc::clone(&_operator);
        handle_connection(arc_operator_clone).await;


        tokio::signal::ctrl_c().await;
    }

    if help_info {
        info!("\nPlease exec: operator -h for help info.\n")
    }
}

pub fn construct_node_config(config_path: PathBuf) -> config::OperatorConfig {
    match config::OperatorConfig::load_config(config_path) {
        Err(OperatorConfigError::ConfigMissing(_)) => {
            error!("config path can't found.");
            std::process::exit(ErrorCodes::PROCESS_EXIT);
        }
        Err(OperatorConfigError::SerializationError(_)) => {
            error!("config file can't be serialize, bad yaml format or incomplete field");
            std::process::exit(ErrorCodes::PROCESS_EXIT);
        }
        Err(OperatorConfigError::IllegalNodeId) => {
            error!("nodeid illegal, must be hex format, and 64 bits");
            std::process::exit(ErrorCodes::PROCESS_EXIT);
        }
        result => result.expect("failed to load zhronod config"),
    }
}

pub async fn build_operator(config: OperatorConfig) -> OperatorArc {
    Operator::operator_factory()
        .set_config(config)
        .initialize_node()
        .await
        .unwrap()
}

async fn cpu_metrics() {
    let mut system = System::new_all();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

    loop {
        interval.tick().await;
        system.refresh_all();

        let memory_usage = system.used_memory() as f64;
        let memory_total= system.total_memory() as f64;
        gauge!("system","memory" => "usage_bytes").set(memory_usage);
        gauge!("system", "memory" => "total_bytes").set(memory_total);

        let cpu_count = system.cpus().len();
        let avg_cpu_usage: f32 = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpu_count as f32;

        gauge!("system", "cpu"=>"usage_percent").set(avg_cpu_usage as f64);
        gauge!("system","cpu"=>"count").set(cpu_count as f64);

    }

}
