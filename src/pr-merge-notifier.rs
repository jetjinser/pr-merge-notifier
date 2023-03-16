use dotenv::dotenv;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::{models::events::payload::PullRequestEventAction, models::repos::GitUser},
    EventPayload,
};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use sendgrid_flows::{send_email, Email};
use slack_flows::send_message_to_channel;
use std::env;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let owner: &str = "jaykchen";
    let repo: &str = "vitesse-lite";
    let sender_email_sendgrid: &str = "jaykchen@gmail.com";

    listen_to_event(owner, repo, vec!["pull_request"], |payload| {
        handler(owner, sender_email_sendgrid, payload)
    })
    .await;

    Ok(())
}

async fn handler(owner: &str, sender_email_sendgrid: &str, payload: EventPayload) {
    dotenv().ok();
    let octocrab = get_octo(Some(String::from(owner)));

    let mut contributor = "".to_string();
    let mut html_url = "".to_string();
    let mut contributor_email = "".to_string();

    if let EventPayload::PullRequestEvent(e) = payload {
        if e.action != PullRequestEventAction::Closed {
            return;
        }
        let pull = e.pull_request;
        html_url = pull.html_url.expect("no html_url found").to_string();
        contributor = pull.user.clone().expect("no contributor info found").login;
        let contributor_url = pull.user.expect("no contributor info found").url.to_string();

        match pull.merge_commit_sha {
            Some(_) => {
                send_message_to_channel("ik8", "general", contributor_url.to_string());

                let token = env::var("GITHUB_TOKEN").unwrap();

let uri = Uri::try_from(contributor_url.as_str()).unwrap();

                let mut writer = Vec::new();
                _ = Request::new(&uri)
                    .method(Method::GET)
                    .header("Accept", "application/vnd.github+json")
                    .header("User-Agent", "Github Connector of Second State Reactor")
                    .header("Authorization", &format!("Bearer {}", token))
                    .send(&mut writer)
                    .map_err(|_e| {})
                    .unwrap();

                let text = String::from_utf8_lossy(&writer);

                let git_user_obj: GitUser = serde_json::from_str(&text).map_err(|_e| {}).unwrap();
                contributor_email = git_user_obj.email;
            }
            None => {
                return;
            }
        };
    }

    send_message_to_channel("ik8", "general", contributor_email.to_string());

    let text = format!(
        r#"
Hi {contributor}, <br/>
Welcome to the {html_url} community, thank you for your contribution!"#
    );
    let email_obj = Email {
        to: vec![contributor_email.to_string()],
        subject: String::from("Thank you for contributing to this repository"),
        content: text,
    };
    send_email(sender_email_sendgrid, &email_obj).expect("failed to send email");
}
