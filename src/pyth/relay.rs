use pyth_sdk_solana::state::SolanaPriceAccount;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

pub const RPC_URL: &str = "https://pythnet.rpcpool.com";

pub fn get_price_feed(key: &str) -> Result<()> {
    let clnt = RpcClient::new(url.to_string());
    let eth_price_key = Pubkey::from_str(key).unwrap();

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let maybe_price = eth_price_feed.get_price_no_older_than(current_time, 60);

    match maybe_price {
        Some(p) => {
            println!("price ........... {} x 10^{}", p.price, p.expo);
            println!("conf ............ {} x 10^{}", p.conf, p.expo);
            return p.price
        }
        None => {
            println!("price ........... unavailable");
            println!("conf ............ unavailable");
        }
    }
}

