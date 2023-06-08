use dotenv::dotenv;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::{
        models::events::payload::PullRequestEventAction, models::repos::GitUser,
        Result as OctoResult,
    },
    EventPayload,
    GithubLogin::Default,
};
use sendgrid_flows::{send_email, Email};
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    let github_owner = env::var("github_owner").unwrap_or("alabulei1".to_string());
    let github_repo = env::var("github_repo").unwrap_or("a-test".to_string());

    listen_to_event(
        &Default,
        &github_owner,
        &github_repo,
        vec!["pull_request"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let sendgrid_token_name =
        env::var("sendgrid_token_name").unwrap_or("jaykchen@gmail.com".to_string());
    let octocrab = get_octo(&Default);

    if let EventPayload::PullRequestEvent(e) = payload {
        if e.action != PullRequestEventAction::Closed {
            return;
        }
        let pull = e.pull_request;
        let html_url = pull.html_url.expect("no html_url found").to_string();

        let user = pull.user.expect("no contributor info found");
        let contributor = user.login;
        let contributor_route = format!("users/{contributor}");

        if pull.merge_commit_sha.is_some() || pull.commits_url.is_some() {
            let response: OctoResult<GitUser> = octocrab.get(&contributor_route, None::<&()>).await;
            let contributor_email = match response {
                Err(_) => "".to_string(),
                Ok(user_obj) => user_obj.email,
            };

            let content = format!(
                r#"
Hi {contributor}, <br/>
Welcome to the {html_url} community, thank you for your contribution!"#
            );
            let email_obj = Email {
                to: vec![contributor_email.to_string()],
                subject: String::from("Thank you for contributing to this repository"),
                content: content,
            };
            send_email(&sendgrid_token_name, &email_obj).expect("failed to send email");
        }
    }
}
