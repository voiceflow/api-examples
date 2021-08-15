# meant for Python 3, will not work with Python 2
import requests # pip install requests

api_key = 'YOUR_API_KEY_HERE' # it should look like this: VF.XXXXXXX.XXXXXX... keep this a secret!
version_id = 'YOUR_VERSION_ID_HERE' # your voiceflow project versionID, find it under the 'integrations' tab

def interact(user_id, request):
    # Start a conversation
    response = requests.post(
        f'https://general-runtime.voiceflow.com/state/{version_id}/user/{user_id}/interact',
        json={ 'request': request },
        headers={ 'Authorization': api_key },
    )

    for trace in response.json():
        if trace['type'] == 'speak' or trace['type'] == 'text':
            print(trace['payload']['message'])
        elif trace['type'] == 'end':
            # an end trace means the the voiceflow dialog has ended
            return False
    return True

name = input('> What is your name?\n')
isRunning = interact(name, { 'type': 'launch' })

while (isRunning):
    nextInput = input('> Say something\n')
    # send a simple text type request with the user input
    isRunning = interact(name, { 'type': 'text', 'payload': nextInput })

input('The end! Start me again with `python index.py` or `python3 index.py`')