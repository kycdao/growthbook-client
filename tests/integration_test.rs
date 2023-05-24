use growthbook_client::*;
use test_log::test;

const KYCDAO_URL: &str = "https://growthbook-proxy.kycdao.xyz/api/";
const SDK_TOKEN: &str = "REPLACE_ME";

fn get_client() -> Client {
    Client::new(KYCDAO_URL, SDK_TOKEN).unwrap()
}

#[test(tokio::test)]
#[ignore]
async fn get_features() {
    let client = get_client();
    let resp = client.get_features().await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn get_feature() {
    let client = get_client();
    let resp = client.get_feature("admin-interface").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn is_enabled() {
    let client = get_client();
    let resp = client.is_enabled("admin-interface").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}
