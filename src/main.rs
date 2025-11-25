use azure_devops_boards_mcp_rust::azure::client::AzureDevOpsClient;
use azure_devops_boards_mcp_rust::mcp::server::AzureMcpServer;
use azure_devops_boards_mcp_rust::server::http;
use clap::Parser;
use rmcp::ServiceExt;
use rmcp::transport::stdio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in server mode
    #[arg(long)]
    server: bool,

    /// Port to run the server on
    #[arg(long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    let client = AzureDevOpsClient::new();
    let mcp_server = AzureMcpServer::new(client);

    if args.server {
        log::info!("Starting web server on port {}", args.port);
        http::run_server(mcp_server, args.port).await?;
    } else {
        log::info!("Starting stdio server");
        let service = mcp_server.serve(stdio()).await?;
        service.waiting().await?;
    }

    Ok(())
}
