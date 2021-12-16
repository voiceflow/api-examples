# Voiceflow API Deno Example

Set up your Voiceflow project with Deno in a matter of minutes.
This example can be adapted to run as an HTTP server.

This example lets you chat with your project via the terminal.

## Documentation

For additional information about the Voiceflow API, visit the [documentation](https://www.voiceflow.com/api/dialog-manager).

## Setup

1. Download and install [Deno](https://deno.land/#installation)
1. Replace `'YOUR_API_KEY_HERE'` in `main.ts` with your Dialog Manager API Key. You can find it under the integrations tab:
   <img src="https://user-images.githubusercontent.com/5643574/129422436-04d964d3-85a0-402d-ae5e-d6e84723da5e.png" width=800 />
1. Run `deno run --allow-net main.ts` to start your chat

## Example

What it might look like in action:

```
$ deno run --allow-net main.ts

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
The end!
```