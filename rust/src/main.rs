use promptly::prompt;
use reqwest::{blocking, header};
use std::error;
use voiceflow::*;

const API_KEY: &str = "YOUR_API_KEY_HERE"; // it should look like this: VF.XXXXXXX.XXXXXX... keep this a secret!
const VERSION_ID: &str = "YOUR_VERSION_ID_HERE"; // your Voiceflow project versionID, find it under the "integrations" tab

const API_URL: &str = "https://general-runtime.voiceflow.com";

fn interact<R: Request>(user_id: &str, request: R) -> Result<bool, Box<dyn error::Error>> {
    let url = format!("{}/state/{}/user/{}/interact", API_URL, VERSION_ID, user_id);

    // call the Voiceflow API with the user's name & request, get back a response
    let client = blocking::Client::new();
    let response = client
        .post(url)
        .json(&request.body())
        .header(header::AUTHORIZATION, API_KEY)
        .send()
        .and_then(blocking::Response::error_for_status)?;

    let traces: Vec<Trace> = response.json()?;

    for trace in traces {
        match trace.r#type.as_str() {
            "text" | "speak" => println!("{}", trace.message()?),
            "end" => return Ok(false),
            _ => {}
        }
    }

    Ok(true)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let user_id: String = prompt("> What is your name?")?;
    // send a simple launch request starting the dialog
    let mut is_running = interact(&user_id, LaunchRequest)?;

    while is_running {
        let next_input = prompt("> Say something")?;
        // send a simple text type request with the user input
        is_running = interact(&user_id, TextRequest::new(next_input))?;
    }

    println!("The end!");

    Ok(())
}
