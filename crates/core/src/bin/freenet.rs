use anyhow::Context;
use clap::Parser;
use freenet::{
    config::{Config, ConfigArgs},
    local_node::{Executor, NodeConfig, OperationMode},
    run_local_node, run_network_node,
    server::serve_gateway,
};
use std::sync::Arc;

async fn run(config: Config) -> anyhow::Result<()> {
    match config.mode {
        OperationMode::Local => run_local(config).await,
        OperationMode::Network => run_network(config).await,
    }
}

async fn run_local(config: Config) -> anyhow::Result<()> {
    tracing::info!("Starting freenet node in local mode");
    let socket = config.ws_api;

    let executor = Executor::from_config(Arc::new(config), None)
        .await
        .map_err(anyhow::Error::msg)?;

    run_local_node(executor, socket)
        .await
        .map_err(anyhow::Error::msg)
}

async fn run_network(config: Config) -> anyhow::Result<()> {
    tracing::info!("Starting freenet node in network mode");

    let clients = serve_gateway(config.ws_api).await;
    tracing::info!("Initializing node configuration");

    let node_config = NodeConfig::new(config)
        .await
        .with_context(|| "failed while loading node config")?;

    let node = node_config
        .build(clients)
        .await
        .with_context(|| "failed while building the node")?;

    run_network_node(node).await
}

fn main() -> anyhow::Result<()> {
    freenet::config::set_logger(None, None);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(
            std::thread::available_parallelism()
                .map(usize::from)
                .unwrap_or(1),
        )
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async move {
        let config = ConfigArgs::parse();
        if config.version {
            println!("Freenet version: {}", config.current_version());
            return Ok(());
        }
        let config = config.build().await?;
        run(config).await
    })?;
    Ok(())
}
