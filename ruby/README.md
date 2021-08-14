# Voiceflow API Node.js Example

Set up your Voiceflow project with Ruby in a matter of minutes. This example can be adapted to run as a server.

This example enables you to chat with your project on the terminal.

## Documentation

For additional information about the Voiceflow API, visit the [documentation](https://www.voiceflow.com/api/dialog-manager).

## Setup

1. If you do not have ruby, install ruby and [bundler](https://bundler.io/) (ruby gems/dependencies manager) from [ruby-lang.org](https://www.ruby-lang.org/en/documentation/installation/), or follow an equivalent guide.
2. In this folder, run `bundle install`.
3. Replace `'YOUR_API_KEY_HERE'` and `'YOUR_VERSION_ID_HERE'` in `index.rb` with your API Key and Voiceflow project version ID. You can find them under the integrations tab:

<img src="https://user-images.githubusercontent.com/5643574/129422436-04d964d3-85a0-402d-ae5e-d6e84723da5e.png" width=800 />

4. run `bundle exect ruby main.rb` to start your chat!

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
