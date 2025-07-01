use nostr::Url;

fn main() {
    let server = "https://nosto.re";
    let mut url = Url::parse(server).unwrap();
    let mut qm = url.query_pairs_mut();
    qm.append_pair("key1", "value1");
    qm.append_pair("key2", "value2");
    qm.append_pair("key3", "value3");
    drop(qm);

    println!("{url}");
}
