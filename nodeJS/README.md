# Voiceflow API Node.js Example
Set up your Voiceflow project with Node.js in a matter of minutes. Node.js is a backend/server runtime that uses Javascript. This example can be adapted to run as a server.

This example enables you to chat with your project on the terminal.

## Setup
1. If you do not have node, install *Node.js* and *npm* (node package manager) from [nodejs.org](https://nodejs.org/), or follow an equivalent guide.
2. In this folder, run `npm install`.
3. Replace `'YOUR_API_KEY_HERE'` and `'YOUR_VERSION_ID_HERE'` in `index.js` with your API Key and Voiceflow project version ID. You can find them under the integrations tab: ![Screen Shot 2021-08-13 at 6 00 07 PM](https://user-images.githubusercontent.com/5643574/129422436-04d964d3-85a0-402d-ae5e-d6e84723da5e.png)
4. run `npm run start` to start your chat!

## Example
What it looks like in action:
```
➜  nodeJS git:(master) ✗ npm run start

> voiceflow-api-nodejs-example@1.0.0 start
> node index.js

> What is your name?: tyler
what can I do for you?
...
> Say something: send email
who is the recipient?
...
> Say something: tyler@voiceflow.com
what is the title of your email?
...
> Say something: How was your day?
sending the email for tyler@voiceflow.com called "How was your day?". Is that correct?
...
> Say something: yes
successfully sent the email for tyler@voiceflow.com called "How was your day?"
The end! Start me again with `npm run start`
```