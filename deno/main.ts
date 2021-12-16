// Import types using skypack for deno support
import { Request } from "https://cdn.skypack.dev/@voiceflow/base-types";

const API_KEY = "YOUR_API_KEY_HERE"; // it should look like this: VF.DM.XXXXXXX.XXXXXX... keep this a secret!

const API_URL = `https://general-runtime.voiceflow.com/`;

// send an interaction to the Voiceflow API, and log the response, returns true if there is a next step
async function interact<R extends typeof Request.BaseRequest>(
  userID: string,
  request: R,
): Promise<boolean> {
  const url = new URL(`state/user/${userID}/interact`, API_URL);

  // call the Voiceflow API with the user's name & request, get back a response
  const response = await fetch(url, {
    method: "POST",
    headers: {
      Authorization: API_KEY,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({request}),
  });

  if (!response.ok) {
    console.log("Uh oh", response.status, response.statusText);
    return false;
  }

  const json = await response.json();
  console.log('response', json);
  // loop through the response
  for (const trace of json) {
    switch (trace.type) {
      case "text":
      case "speak": {
        console.log(trace.payload.message);
        break;
      }
      case "end": {
        // an end trace means the the Voiceflow dialog has ended
        return false;
      }
    }
  }

  return true;
}

const userID = prompt("> What is your name?");
if (userID) {
  // send a simple launch request starting the dialog
  let isRunning = await interact<typeof Request.LaunchRequest>(userID, {
    type: Request.RequestType.LAUNCH,
  });

  while (isRunning) {
    const nextInput = prompt("> Say something");
    if (!nextInput) break;

    // send a simple text type request with the user input
    isRunning = await interact<typeof Request.TextRequest>(userID, {
      type: Request.RequestType.TEXT,
      payload: nextInput,
    });
  }
}

console.log("The end!");
