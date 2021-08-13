// view the README.md file for additional instructions
const axios = require('axios');
const { cli } = require('cli-ux');

const API_KEY = 'YOUR_API_KEY_HERE'; // it should look like this: VF.XXXXXXX.XXXXXX... keep this a secret!
const VERSION_ID = 'YOUR_VERSION_ID_HERE'; // your voiceflow project versionID, find it under the "integrations" tab

// send an interaction to the voiceflow API, and log the response, returns true if there is a next step
async function interact(userID, request){
    console.log('...');
    // call the voiceflow api with the user's name & request, get back a response
    const response = await axios({
        method: 'POST',
        url: `https://general-runtime.voiceflow.com/state/${VERSION_ID}/user/${userID}/interact`,
        headers: { Authorization: API_KEY },
        data: { request },
    });
    // loop through the response
    for (const trace of response.data) {
        if (trace.type === 'speak') {
            console.log(trace.payload.message);
        } else if (trace.type === 'text') {
            console.log(trace.payload.text);
        } else if (trace.type === 'end') {
            // an end trace means the the voiceflow dialog has ended
            return false;
        }
    }
    return true;
}

async function main() {
    const userID = await cli.prompt('> What is your name?');
    // send a simple launch request starting the dialog
    let isRunning = await interact(userID, { type: 'launch' });

    while (isRunning) {
        const nextInput = await cli.prompt('> Say something');
        // send a simple text type request with the user input
        isRunning = await interact(userID, { type: 'text', payload: nextInput });
    }
    console.log("The end! Start me again with `npm run start`")
}
main();