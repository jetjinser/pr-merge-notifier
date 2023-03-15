use github_flows::octocrab::Result as OctoResult;
use github_flows::{get_octo, listen_to_event, EventPayload};
use sendgrid_flows::{send_email, Email};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let owner: &str = "jaykchen";
    let repo: &str = "vitesse-lite";
    let sender_email_sendgrid : &str = "jaykchen@gmailcom";

    listen_to_event(owner, repo, vec!["pull_request"], |payload| {
        handler(owner, sender_email_sendgrid, payload)
    })
    .await;

    Ok(())
}

async fn handler(owner: &str, sender_email_sendgrid: &str, payload: EventPayload) {
    let octocrab = get_octo(Some(String::from(owner)));

    let mut user = "".to_string();
    let mut html_url = "".to_string();
    let mut contributor_email = "".to_string();

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
                        contributor_email = user_obj["email"]
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
        to: vec![contributor_email.to_string()],
        subject: String::from("Thank you for contributing to this repository"),
        content: text,
    };
    send_email(sender_email_sendgrid, &email_obj).expect("failed to send email");
}
