# <p align="center">Send the contributor an email when the PR is merged</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/new">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>


[Deploy this function on flows.network](#deploy-pr-merge-notifier-on-your-github-repo), and you will automate the work to get reach out to the contributors. When the Pull Requests is merged by miantianers, send a thank you email to the contributors. 

> This function works when the contributor has a plublic email in his/her profile page. If the contributor sets email address private, then we can't send him an email.

![image](https://user-images.githubusercontent.com/45785633/228182641-835276f6-7aa9-48c0-a16b-3ef9cf452d30.png)

## Prerequisite 

You will need a [Sendgrid API key](https://app.sendgrid.com/settings/api_keys). If you do not already have one, [sign up here](https://app.sendgrid.com/settings/api_keys).


## Deploy PR Merge Notifier on your GitHub repo

To install this PR merge notifier app, we use [flows.network](https://flows.network/), a serverless platform that lets you make a workflow app in three simple steps.

### Fork this repo and make simple code edit

Fork [this repo](https://github.com/flows-network/pr-merge-notifier) and open the source code. Replace the parameters in the red boxes below with your personal GitHub account, the GitHub Repo owner and repo name where you want to install the app and then your sendgrid account respectively.

![image](https://user-images.githubusercontent.com/45785633/228185236-b09607c6-6d40-440b-9677-59f2bf96eb97.png)


### Deploy the code on flows.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying this app.
3. Authorize the [flows.network](https://flows.network/) to access the `pr-merge-notifier` repo you just forked. 

<img width="915" alt="image" src="https://user-images.githubusercontent.com/45785633/227570338-eadbd41e-8d57-4d47-bb69-e3e805444bec.png">

4. Click the Deploy button to deploy your function.

### Configure SaaS integrations

Next, flows.network directs you to configure the SaaS integrations required by your flow.

<img width="927" alt="image" src="https://user-images.githubusercontent.com/45785633/227570411-aaa84463-7f2e-47a9-9e69-15ed585a478a.png">

Here we need to configue 2 SaaS integrations.

1. Click the "Connect/+ Add new authentication" button to authenticate your **GitHub account**. You'll be redirected to a new page to grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on the repo that you changed in the code above.
2. Click the "Connect/+ Add new authentication" button to authenticate your **Sendgrid account**. You'll be redirected to a new page where you could copy and paste your SendGrid API key and then name the key. **Note that the name you enter here should be the same as the name in the code above.**
  
<img width="765" alt="image" src="https://user-images.githubusercontent.com/45785633/227570457-94ad1092-e483-436c-be4e-624d1faff18a.png">


After that, click the Check button to see your flow details. As soon as the flow function's status turns `ready` and the flow's status becomes `running`, the PR merge notifier goes live. Get connected with your contributor right away as contributors increase!


> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!


## Others


To build locally, make sure you have installed Rust and added `wasm32-wasi` target.

```
cargo build target wasm32-wasi --release
```
