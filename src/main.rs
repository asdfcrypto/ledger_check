use anyhow::Result;
use ethers::{providers::{Http, Provider, Middleware}, signers::{Ledger, HDPath}, utils::format_units};

async fn print_ledger(client: &Provider<Http>, index: usize, hd_path: HDPath) -> Result<()> {
    let ledger = Ledger::new(hd_path, 1).await?;
    let address = ledger.get_address().await?;
    let balance = client.get_balance(address, None).await?;
    println!("{} https://etherscan.io/address/{:?} {}", index, address, format_units(balance, "eth")?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Provider::<Http>::try_from("http://127.0.0.1:8545")?;

    println!("LedgerLive");
    for i in 0..20 {
        print_ledger(&client, i, HDPath::LedgerLive(i)).await?;
    }

    println!("Legacy");
    for i in 0..20 {
        print_ledger(&client, i, HDPath::Legacy(i)).await?;
    }
    Ok(())
}
