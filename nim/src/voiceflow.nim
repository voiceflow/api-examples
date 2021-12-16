# view the README.md file for additional instructions

import httpclient
import rdstdin
import json
import strformat

const apiKey = "YOUR_API_KEY_HERE" # it should look like this: VF.DM.XXXXXXX.XXXXXX... keep this a secret!

var http = newHttpClient()

proc interact(userID: string, request: JsonNode): bool =
  echo "..."

  let headers = newHttpHeaders({"Content-Type": "application/json", "Authorization": apiKey})
  let url = fmt"https://general-runtime.voiceflow.com/state/user/{userID}/interact"

  # call the Voiceflow API with the user's name & request, get back a response
  let response = http.request(url, HttpPost, $(%*request), headers)

  let body = response.body().parseJson()

  if response.status != $Http200:
    # quit on errors
    quit(fmt"HTTP error: {body}", QuitFailure)

  let traces = body.getElems()
  for trace in traces:
    case trace["type"].getStr():
      of "speak":
        echo trace["payload"]["message"].getStr()
      of "text":
        echo trace["payload"]["text"].getStr()
      of "end":
        # an end trace means the the Voiceflow dialog has ended
        return false

  return true

let userID = readLineFromStdin("> What is your name?: ")
# send a simple launch request starting the dialog
var isRunning = userID.interact(%*{"type": "launch"})

while isRunning:
  # send a simple text type request with the user input
  let nextInput = readLineFromStdin("> Say something: ")
  isRunning = userID.interact(%*{"type": "text", "payload": nextInput})

echo "The end!"
