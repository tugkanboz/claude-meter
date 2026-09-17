use anyhow::Result;
use clap::Parser;
use claude_meter::{format, oauth};

#[derive(Parser)]
#[command(
    name = "claude-meter",
    version,
    about = "Live Claude plan usage and extra-usage balance"
)]
struct Cli {
    /// Emit machine-readable JSON
    #[arg(long)]
    json: bool,

    /// Friendly name shown for this credential source
    #[arg(long)]
    label: Option<String>,

    /// macOS Keychain service containing Claude Code credentials
    #[arg(long)]
    keychain_service: Option<String>,

    /// Optional macOS Keychain account used to disambiguate duplicate services
    #[arg(long)]
    keychain_account: Option<String>,

    /// Read a Claude Code .credentials.json file instead of Keychain
    #[arg(long)]
    credentials_file: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Single source: the OAuth token Claude Code stores in the macOS Keychain
    // (service "Claude Code-credentials"). api.anthropic.com exposes the usage
    // data on /api/oauth/usage with no Cloudflare in front. One token = one
    // account (the one the active Claude Code CLI is logged into).
    let source = oauth::CredentialSource {
        label: cli.label,
        keychain_service: cli.keychain_service,
        keychain_account: cli.keychain_account,
        credentials_file: cli.credentials_file,
    };
    let snapshot = oauth::fetch_oauth_snapshot_from(&source).await.map_err(|e| {
        anyhow::anyhow!("oauth: {e:#}. Is Claude Code logged in? Run `claude` once to refresh.")
    })?;

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    } else {
        format::print_pretty(&snapshot);
    }
    Ok(())
}
