# Voiceflow API Nim Example

Set up your Voiceflow project with Nim in a matter of minutes.
[Nim](https://nim-lang.org/) is a statically typed compiled systems programming language.

This example lets you chat with your project via the terminal.

## Documentation

For additional information about the Voiceflow API, visit the [documentation](https://www.voiceflow.com/api/dialog-manager).

## Setup

1. If you do not yet have Nim installed, [install it from here](https://nim-lang.org/install.html).
2. Replace `'YOUR_API_KEY_HERE'` in `src/voiceflow.nim` with your Dialog Manager API Key. You can find it under the integrations tab:

   <img src="https://user-images.githubusercontent.com/5643574/129422436-04d964d3-85a0-402d-ae5e-d6e84723da5e.png" width=800 />

3. Run `nimble run` to compile and run the app.

## Example

What it might look like in action:

```
$ nimble run

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
