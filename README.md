# Send the contributor an email when the PR is merged

[Deploy this function on flows.network](#deploy-pr-merge-notifier-on-your-github-repo), and you will automate the work to get reach out to the contributors. When the Pull Requests is merged by miantianers, send a thank you email to the contributors. 

> This only works when the contributor has a plublic email in his/her profile page.

![image](https://user-images.githubusercontent.com/45785633/228182641-835276f6-7aa9-48c0-a16b-3ef9cf452d30.png)




<img width="915" alt="image" src="https://user-images.githubusercontent.com/45785633/227570338-eadbd41e-8d57-4d47-bb69-e3e805444bec.png">



<img width="927" alt="image" src="https://user-images.githubusercontent.com/45785633/227570411-aaa84463-7f2e-47a9-9e69-15ed585a478a.png">


<img width="765" alt="image" src="https://user-images.githubusercontent.com/45785633/227570457-94ad1092-e483-436c-be4e-624d1faff18a.png">


## Deploy PR Merge Notifier on your GitHub repo

To install this PR merge notifier app, we use [flows.network](https://flows.network/), a serverless platform that lets you make a workflow app in three simple steps.

### Fork this repo and make simple code edit

Fork [this repo](https://github.com/flows-network/pr-merge-notifier) and open the source code. Replace the parameters in the red boxes below with your personal GitHub account, the GitHub Repo owner and repo name where you want to install the app and then your sendgrid account respectively.



![image](https://user-images.githubusercontent.com/45785633/228185236-b09607c6-6d40-440b-9677-59f2bf96eb97.png)

Fork [this repo](https://github.com/flows-network/pr-merge-notifier) and open the source code. Replace the parameters in the red boxes below with your GitHub Repo owner and repo name, and then your Slack workspace and channel respectively.

<img width="743" alt="image" src="https://user-images.githubusercontent.com/45785633/227526953-210e366f-9599-4344-b213-d0ff4f185964.png">



### Deploy the code on flows.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying the Github Star Slack Messenger
3. Authorize the [flows.network](https://flows.network/) to access the `slack-calculator` repo you just forked. 

![](https://i.imgur.com/uk1FYW1.png)

4. Click the Deploy button to deploy your function.

### Configure SaaS integrations

Next, flows.network directs you to configure the SaaS integrations required by your flow.

![](https://i.imgur.com/VDhVeLB.png)

Here we need to configue 2 SaaS integrations.

1. Click the "Connect/+ Add new authentication" button to authenticate your **Slack account**. You'll be redirected to a new page to grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on your Slack workspace. The workspace is the parameter you entered at the first step. (Here as can be seen in the screenshot, I already connected the Slack integration)
2.  Click the "Connect/+ Add new authentication" button to authenticate your **GitHub account**. You'll be redirected to a new page to grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on the repo that you changed in the code above.




After that, click the Check button to see your flow details. As soon as the flow function's status turns `ready` and the flow's status becomes `running`, the Github Star Slack Messenger goes live. Get updates right away as your GitHub stars increase!



> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!


## Others


To build locally

```
cargo build target wasm32-wasi --release
```
