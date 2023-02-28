use anyhow::{Error, Result};
use github_flows::{get_octo, listen_to_event, EventPayload};

use slack_flows::send_message_to_channel;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event("jaykchen", "vitesse-lite", vec!["pull_request"], handler).await;

    Ok(())
}

async fn get_email(user: &str) {
    let octo = get_octo(Some("jaykchen".to_string()));
    let query_str = format!("https://api.github.com/users/{user}");
    send_message_to_channel("ik8", "general", "ready to query".to_string());

    let response: serde_json::Value = octo
        ._get(query_str, None::<&()>)
        .await
        .expect("got no response")
        .json()
        .await
        .expect("failed to parse response to json");

    send_message_to_channel("ik8", "general", response.to_string());

    // match response.get("email") {
    //     Some(e) => Some(e.to_string()),
    //     None => None,
    // }
}

async fn handler(payload: EventPayload) {
    let mut title: String = "no title found".to_string();
    let mut merged: bool = false;
    let mut html_url: String = "no html_url found".to_string();
    let mut user: String = "".to_string();

    if let EventPayload::PullRequestEvent(e) = payload {
        let pr = e.pull_request;
        title = pr.title.expect("no title");
        html_url = pr.html_url.expect("no html_url field").to_string();
        user = pr.user.expect("user not found").login;
        merged = match pr.merge_commit_sha {
            Some(_sha) => true,
            None => false,
        };
    }

    get_email(&user).await;

    // match get_email(&user).await {
    //     None => {}
    //     Some(email) => {
    let merged_str = if merged { "merged" } else { "null" };
    let text = format!("title: {title}\n html_url: {html_url}\n user: {user}\n, email: email\n merged: {merged_str}");
    send_message_to_channel("ik8", "general", text);
    // }
    // }
}
