require "faraday"
require "json"

API_KEY = "YOUR_API_KEY_HERE" # it should look like this: VF.XXXXXXX.XXXXXX... keep this a secret!
VERSION_ID = "YOUR_VERSION_ID_HERE" # your voiceflow project versionID, find it under the "integrations" tab

def interact(user_id, request)
    puts "..."
    puts user_id, request.to_json

    # call the voiceflow api with the user's name & request, get back a response
    response = Faraday.post(
        "https://general-runtime.voiceflow.com/state/#{VERSION_ID}/user/#{user_id}/interact",
        { 
            headers: {"Content-Type" => "application/json", "Authorization": API_KEY },
            body: request 
        }
    )

    if response.nil?
        return false
    end

    # loop through the response
    response.data.each do |trace|
        # if trace.type == "speak"
        #     puts trace.payload.message
        # elsif trace.type == 'text'
        #     puts trace.payload.text
        # elsif trace.type == 'end'
        #     # an end trace means the the voiceflow dialog has ended
        #     return false
    end

    return true
end

def main()
    puts "> What is your name?"
    user_id = gets.chomp.split(',')[0]

    is_running = interact(user_id, { type: 'launch' })
    
    while is_running do
        puts "> Say something"
        next_input = gets.chomp.split(',')[0] 
        # send a simple text type request with the user input
        is_running = interact(user_id, { type: 'text', payload: next_input });
    end
    
    puts "The end! Start me again with bundle exect ruby main.rb"
end

main()