use avail_rust::{avail, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

use avail::balances::calls::types as BalanceCalls;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest: &str = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let amount = 1_000_000_000_000_000_000u128; // 1 Avail

	let result = sdk
		.tx
		.balances
		.transfer_keep_alive(dest, amount, WaitFor::BlockInclusion, &account)
		.await?;

	let tx = sdk
		.util
		.fetch_transaction::<BalanceCalls::TransferKeepAlive>(result.block_hash, result.tx_hash)
		.await
		.map_err(|e| e.to_string())?;

	println!("Value={:?}", tx.value);
	println!("PalletName={:?}", tx.details.pallet_name());

	Ok(())
}
