use github_flows::{get_octo, listen_to_event, EventPayload};
use sendgrid_flows::{send_email, Email};
use serde_json::Value;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let your_github_login: &str = "jaykchen";
    let your_repo_name: &str = "vitesse-lite";
    listen_to_event(
        your_github_login,
        your_repo_name,
        vec!["pull_request"],
        handler,
    )
    .await;

    Ok(())
}

async fn get_email(user: &String) -> String {
    let octocrab = get_octo(None);
    let query_str = format!("/users/{user}");

    let response = octocrab
        .get(query_str, None::<&()>)
        .await
        .expect("no reponse");

    let parsed: Value = serde_json::from_str(response).expect("failed to parse");

    match parsed["email"].as_str() {
        Some(e) => e.to_string(),
        None => "".to_string(),
    }
}

async fn handler(payload: EventPayload) {
    let mut html_url: String = "no html_url found".to_string();
    let mut user: String = "".to_string();

    if let EventPayload::PullRequestEvent(e) = payload {
        let pr = e.pull_request;
        html_url = pr.html_url.expect("no html_url field").to_string();
        user = pr.user.expect("user not found").login;
        match pr.merge_commit_sha {
            Some(_) => {}
            None => {
                return;
            }
        };
    }

    let email = get_email(&user).await;

    let text = format!(
        r#"
Hi {user}, <br/>
Welcome to the {html_url} community, thank you for your contribution!"#
    );
    let email_obj = Email {
        to: vec![email.clone()],
        subject: String::from("Thank you for contributing to this repository"),
        content: text,
    };
    send_email(&email, &email_obj).expect("failed to send email");
}
