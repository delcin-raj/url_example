use nostr::Url;

fn main() {
    let server = "https://nosto.re";
    let server_url = Url::parse(server).unwrap();
    
    let url_str = format!("{}", server_url);
    let ending_char = url_str.chars().last().unwrap();

    // Trailing '/' is automatically added
    assert_eq!(ending_char, '/');

    println!("{}", format!("{}upload", server_url));
}
