extern crate reqwest;

fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let username = "testuser".to_owned();
    let password: Option<String> = None;
    
    match client
        .get("http://localhost:8080/auth")
        .basic_auth(username, password)
        .send() {
            Ok(response) => println!("We've got this response:\n{:#?}", response),
            Err(error) => println!("Request failed with error: {:?}", error)
        }

    Ok(())
}
