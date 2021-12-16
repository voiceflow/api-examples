use promptly::prompt;
use reqwest::{blocking, header};
use serde_json::{json, Value as JsonValue};

const API_KEY: &str = "YOUR_API_KEY_HERE"; // it should look like this: VF.DM.XXXXXXX.XXXXXX... keep this a secret!

const API_URL: &str = "https://general-runtime.voiceflow.com";

fn interact(user_id: &str, request: JsonValue) -> Result<bool, Box<dyn std::error::Error>> {
    let url = format!("{}/state/user/{}/interact", API_URL, user_id);

    // call the Voiceflow API with the user's name & request, get back a response
    let client = blocking::Client::new();
    let response = client
        .post(url)
        .json(&request)
        .header(header::AUTHORIZATION, API_KEY)
        .send()
        .and_then(blocking::Response::error_for_status)?;

    let traces: Vec<JsonValue> = response.json()?;

    for trace in traces {
        match trace["type"].to_string().as_str() {
            "\"text\"" | "\"speak\"" => println!("{}", trace["payload"]["message"]),
            "\"end\"" => return Ok(false),
            _ => {}
        }
    }

    Ok(true)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_id: String = prompt("> What is your name?")?;
    // send a simple launch request starting the dialog
    let mut is_running = interact(&user_id, json!({"request": {"type": "launch"}}))?;

    while is_running {
        let next_input: String = prompt("> Say something")?;
        // send a simple text type request with the user input
        is_running = interact(
            &user_id,
            json!({"request": {"type": "text", "payload": next_input}}),
        )?;
    }

    println!("The end!");

    Ok(())
}
