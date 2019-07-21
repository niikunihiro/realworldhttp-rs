use reqwest;

fn main() {
    let client = reqwest::Client::new();
    let res = client.get("https://www.rust-lang.org")
        .send();

    let mut res = match res {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error {:?}", e);
            std::process::exit(1);
        }
    };
    let body = match res.text() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error {:?}", e);
            std::process::exit(1);
        }
    };

    println!("body = {:?}", body);
}
