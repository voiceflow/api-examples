# Voiceflow API Python Example

Set up your Voiceflow project with Python in a matter of minutes. This example can be adapted to run as a server with Django or Flask.

This example enables you to chat with your project on the terminal.

## Documentation

For additional information about the Voiceflow API, visit the [documentation](https://www.voiceflow.com/api/dialog-manager).

## Setup

1. If you do not have python, install Python from [python.org](https://www.python.org/downloads), or follow an equivalent guide.
2. Replace `'YOUR_API_KEY_HERE'` in `index.py` with your Dialog Manager API Key. You can find it under the integrations tab:

  <img src="https://user-images.githubusercontent.com/5643574/129422436-04d964d3-85a0-402d-ae5e-d6e84723da5e.png" width=800 />

3. run `python index.py` to start your chat!

(If you have both Python 2 and Python 3 on your computer, you might likely need to run `python3 index.py`).

## Example

What it might look like in action:

```
➜  python git:(master) ✗ python index.py
> What is your name?
Tyler
what can I do for you?
> Say something
send email
who is the recipient?
> Say something
tyler@voiceflow.com
what is the title of your email?
> Say something
hello world
sending the email for tyler@voiceflow.com called hello world , is that correct?
> Say something
yes
successfully sent the email for tyler@voiceflow.com called hello world
The end! Start me again with `python index.py` or `python3 index.py`
```
