use reqwest;
use reqwest::multipart;

fn main() {
//    get();
//    post();
//    post_file();
    head();
}

fn head () {
    let client = reqwest::Client::new();
    let res = client.head("http://localhost:8888/index.php")
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

    println!("{:?}", res.headers());
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

fn post () {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8888/post.php")
        .header("Content-Type", "text/plain")
        .body("the exact body that is sent")
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

fn post_file () {
    let form = multipart::Form::new()
        .text("username", "seanmonster")
        .file("photo", "/Users/nielsen/Pictures/Image.png");

    let form = match form {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error {:?}", e);
            std::process::exit(1);
        }
    };

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8888/post.php")
        .multipart(form)
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