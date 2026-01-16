#![allow(unused)]
// Example of simple client to connect with Kaspa node using wRPC connection and collect some node and network basic data

use kaspa_rpc_core::{api::rpc::RpcApi, GetBlockDagInfoResponse, GetServerInfoResponse};
use kaspa_wrpc_client::{
    client::{ConnectOptions, ConnectStrategy},
    prelude::{NetworkId, NetworkType},
    result::Result,
    KaspaRpcClient, Resolver, WrpcEncoding,
};
use std::process::ExitCode;
use std::time::Duration;
use tokio::time::sleep;
use kaspa_addresses::Address;   //
use kaspa_hashes::Hash;
use std::str::FromStr;

#[tokio::main]
async fn main() -> ExitCode {
    match check_node_status().await {
        Ok(_) => {
            println!("  Well done!");
            ExitCode::SUCCESS
        }
        Err(error) => {
            println!("An error occurred: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn check_node_status() -> Result<()> {
    // Select encoding method to use, depending on node settings
    let encoding = WrpcEncoding::Borsh;

    // If you want to connect to your own node, define your node address and wRPC port using let url = Some("ws://0.0.0.0:17110")
    // Verify your Kaspa node is runnning with --rpclisten-borsh=0.0.0.0:17110 parameter
    // In this example we don't use a specific node but we connect through the resolver, which use a pool of public nodes
    let url = Some("ws://127.0.0.1:17110");
    // let url = None;
    let resolver = Some(Resolver::default());

    // println!("{:#?}", resolver);
    // println!();
    // Define the network your Kaspa node is connected to
    // You can select NetworkType::Mainnet, NetworkType::Testnet, NetworkType::Devnet, NetworkType::Simnet
    let network_type = NetworkType::Mainnet;
    let selected_network = Some(NetworkId::new(network_type));

    // Advanced options
    let subscription_context = None;

    // Create new wRPC client with parameters defined above
    let client = KaspaRpcClient::new(encoding, url, resolver, selected_network, subscription_context)?;

    // println!("{:#?}", client); ///
    // println!();
    // Advanced connection options
    let timeout = 5_000;
    let options = ConnectOptions {
        block_async_connect: true,
        connect_timeout: Some(Duration::from_millis(timeout)),
        strategy: ConnectStrategy::Fallback,
        ..Default::default()
    };

    // Connect to selected Kaspa node
    let deb1 = client.connect(Some(options)).await?;
    // println!("{:#?}", deb1); ///
    // println!();

    // Retrieve and show Kaspa node information
    let GetServerInfoResponse { is_synced, server_version, network_id, has_utxo_index, .. } = client.get_server_info().await?;

    println!("Node version: {server_version}");
    println!("Network: {network_id}");
    println!("Node is synced: {is_synced}");
    println!("Node is indexing UTXOs: {has_utxo_index}");

    // Retrieve and show Kaspa network information
    let GetBlockDagInfoResponse {
        block_count,
        header_count,
        tip_hashes,
        difficulty,
        past_median_time,
        virtual_parent_hashes,
        pruning_point_hash,
        virtual_daa_score,
        sink,
        ..
    } = client.get_block_dag_info().await?;

    println!("Block count: {block_count}");
    println!("Header count: {header_count}");
    // println!("Tip hashes:");
    // for tip_hash in tip_hashes {
    //     println!("{tip_hash}");
    // }
    // println!("Difficulty: {difficulty}");
    // println!("Past median time: {past_median_time}");
    // println!("Virtual parent hashes:");
    // for virtual_parent_hash in virtual_parent_hashes {
    //     println!("{virtual_parent_hash}");
    // }
    println!("Pruning point hash: {pruning_point_hash}");
    println!("Virtual DAA score: {virtual_daa_score}");
    println!("Sink: {sink}");
/*
    let s = "32296d0348b1072ac9e48eaa9b3eeb5654594444d003628fdf5c3fa84c94cd6e";
    let bytes = Hash::from_str(s).expect("wrong format");
    let bl1 = client.get_block(bytes, true).await?;
    println!("{:#?}", bl1);
*/
    // let addresses1 = vec![Address::constructor("kaspasim:qrqkdk9adu8zatc7f0zy949hmdeuglfqmpcd5646r05fj2lmjuf5u6ra6ud8e")];
    let addresses1 = vec![Address::constructor("kaspa:qqwgjtcpwxsk4l4x9a7xxpxdfgvzkqgsl4emvj7yaqtrdlt85dxzctkah7j50")];
    let uts1 = client.get_utxos_by_addresses(addresses1).await?;
    let mut cn = 0u16;
    for u in uts1 {
        cn += 1;
        let en = &u.utxo_entry.block_daa_score;
        if *en >= 3 {
            println!("{:#?}", u);
        }
    }
    println!("utxos= {cn}");

    /* let bcss = client.get_blocks(None, true, true).await?;
    cn = 0;
    for i in bcss.blocks {
        cn += 1;
    }   */
    // println!("blocks= {cn}");

    // let txi1 = Hash::from_str("b95142806ff40bd58e2e6b356280c11248e9e2ebc5dc0f59f0a86fc30ebdb997").expect("wrong format");
    // let uts2 = client.get_utxo_return_address(txi1, 1929).await?;
    // println!("{:#?}", uts2);

    // supply
    let supp1 = client.get_coin_supply().await?;
    let supp01 = supp1.circulating_sompi;
    // println!("{}", supp1.circulating_sompi / 100000000);
    /*
    for i in 1..=10 {
        // print!(" {}", i);
        sleep(Duration::from_secs(30)).await;
        let supp2 = client.get_coin_supply().await?;
        println!("{}", supp2.circulating_sompi / 100000000);
    }

    let supp2 = client.get_coin_supply().await?;
    let supp02 = supp2.circulating_sompi;
    let supdef = (supp02 - supp01) / 100000000;
    let supdf1 = supdef as f64 / 300.0;

    // println!();
    println!("in sec.  {} KAS", supdf1);    */

    //  client.shutdown().await?;  //

    // Disconnect client from Kaspa node
    client.disconnect().await?;

    // Return function result
    Ok(())
}
