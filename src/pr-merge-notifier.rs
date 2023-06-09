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
use flowsnet_platform_sdk::logger;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
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
        env::var("sendgrid_sender_address").unwrap_or("jaykchen@gmail.com".to_string());
    let octocrab = get_octo(&Default);

    if let EventPayload::PullRequestEvent(e) = payload {
        if e.action != PullRequestEventAction::Closed {
            return;
        }
        let pull = e.pull_request;
        let html_url = pull.html_url.expect("no html_url found").to_string();

        let user = pull.user.expect("no contributor info found");
        log::debug!("use is: {:?}", user);
        let contributor = user.login;
        let contributor_route = format!("users/{contributor}");
        log::debug!("contributor route is: {}", contributor_route);

        if pull.merge_commit_sha.is_some() || pull.commits_url.is_some() {
            let response: OctoResult<GitUser> = octocrab.get(&contributor_route, None::<&()>).await;
            let contributor_email = match response {
                Err(_) => "".to_string(),
                Ok(user_obj) => user_obj.email,
            };
            log::debug!("contributor email is {}", contributor_email);

            let content = format!(
                r#"
Hi {contributor}, <br/>
We would like to extend a warm welcome to the WasmEdge community, and express our gratitude for your contribution! Your efforts are greatly appreciated, and we are thrilled to have you on board. {html_url} <br/>
Please take a moment to fill out this Google form, and we will send you some WasmEdge SWAG as a token of our appreciation for your hard work:
https://github.com/WasmEdge/WasmEdge/issues/551 <br/>
We also invite you to join our Discord server, where you will have the opportunity to connect with other developers and contribute to the WasmEdge community. To join, please visit:
https://github.com/WasmEdge/WasmEdge#contact <br/>
Cheers,
Vivian "#
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
