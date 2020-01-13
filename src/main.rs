extern crate komodo_airdrop;
extern crate komodo_rpc_client;
extern crate csv;
extern crate serde;
extern crate chrono;

use komodo_airdrop::Snapshot;
use komodo_rpc_client::Chain;
use std::collections::HashMap;
use csv::Error;

fn main() {
    let snapshot = Snapshot::builder()
        .on_chain(Chain::Custom("ILN".to_string()))
        .using_threshold(0.4999)
        .build()
        .expect("Failed to create snapshot of ILN chain");

    let mut hashm = HashMap::new();
    snapshot.addresses.iter().for_each(|address| {
        hashm.insert(address.addr.clone(), address.amount);
    });

    write_hashmap_to_csv(&hashm).expect("Problem during CSV creation");
}

fn write_hashmap_to_csv(map: &HashMap<String, f64>) -> Result<(), Error> {
    let date = chrono::Utc::now();
    let date = date.format("%Y-%m-%d").to_string();

    let mut writer = csv::Writer::from_path(format!("./snapshot_{}.csv", date)).expect("Problem while creating snapshot file");

    map.iter().for_each(|(key, value)| {
        writer.serialize((key, value)).expect("Problem during serialization");
    });

    Ok(())
}
