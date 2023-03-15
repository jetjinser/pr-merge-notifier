use github_flows::octocrab::Result as OctoResult;
use github_flows::{get_octo, listen_to_event, EventPayload};
use sendgrid_flows::{send_email, Email};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let owner: &str = "jaykchen";
    let repo: &str = "vitesse-lite";

    listen_to_event(owner, repo, vec!["pull_request"], |payload| {
        handler(owner, payload)
    })
    .await;

    Ok(())
}

async fn handler(owner: &str, payload: EventPayload) {
    let octocrab = get_octo(Some(String::from(owner)));

    let mut user = "".to_string();
    let mut html_url = "".to_string();
    let mut email = "".to_string();

    if let EventPayload::PullRequestEvent(e) = payload {
        let pull = e.pull_request;
        html_url = pull.html_url.unwrap().to_string();
        user = pull.user.unwrap().login;

        match pull.merge_commit_sha {
            Some(_) => {
                let query_str = format!("/users/{user}");

                let response: OctoResult<serde_json::Value> =
                    octocrab.get(query_str, None::<&()>).await;
                match response {
                    Err(_) => {}
                    Ok(user_obj) => {
                        email = user_obj["email"]
                            .as_str()
                            .unwrap_or("contributor email not found")
                            .to_string();
                    }
                }
            }
            None => {
                return;
            }
        };
    }

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
