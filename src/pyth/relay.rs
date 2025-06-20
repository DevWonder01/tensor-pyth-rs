use pyth_sdk_solana::state::SolanaPriceAccount;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

pub const RPC_URL: &str = "https://pythnet.rpcpool.com";

pub fn get_price_feed(key: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let clnt = RpcClient::new(RPC_URL.to_string());
    let eth_price_key = Pubkey::from_str(key).unwrap();

    let mut eth_price_account = clnt.get_account(&eth_price_key).unwrap();
    let eth_price_feed =
        SolanaPriceAccount::account_to_feed(&eth_price_key, &mut eth_price_account).unwrap();

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let maybe_price = eth_price_feed.get_price_no_older_than(current_time, 60);

    match maybe_price {
        Some(p) => {
            println!("price ........... {} x 10^{}", p.price, p.expo);
            println!("conf ............ {} x 10^{}", p.conf, p.expo);
            Ok(p.price)
        }
        None => {
            println!("price ........... unavailable");
            println!("conf ............ unavailable");
            Err("no price".into())
        }
    }
}
