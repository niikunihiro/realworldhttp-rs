use reqwest;

fn main() {
    get();
}

fn get () {
    let client = reqwest::Client::new();
    let res = client.get("http://localhost:8888/index.php")
        .query(&[("lang", "rust")])
        .send();

    let mut res = match res {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error {:?}", e);
            std::process::exit(1);
        }
    };

    if ! res.status().is_success() {
        std::process::exit(1);
    }

    let body = match res.text() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error {:?}", e);
            std::process::exit(1);
        }
    };

    println!("body = {:?}", body);
}
