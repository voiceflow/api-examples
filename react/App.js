import './App.css';
import React, {useState} from "react";

const API_KEY = 'YOUR_API_KEY_HERE'; // change this to YOUR API key, it should look like this: VF.DM.XXXXXXX.XXXXXX... keep this a secret!

const RUNTIME_API_URL = 'https://general-runtime.voiceflow.com/' // change this to whatever your API URL is

const App = props => {
  
  const [user, setUser] = useState("");
  const [message, setMsg] = useState("");
  const [resArr, updateArr] = useState([]);
  
  const handleClick = async() => {
    let data = await fetch(RUNTIME_API_URL, { 
      headers: { Authorization: apiKey, 'Content-Type': 'application/json' },
      method: "POST",
      body: JSON.stringify({
        "request": {
          "type": "text",
          "payload": message
        }
      })
    })
    let postRes = await data.json();
    console.log(postRes);
    for(let i=0; i < postRes.length; i++){
      if(postRes[i].type === "text"){
        updateArr(arr => [...arr, message + ">" + postRes[i].payload.message]);
      }
    }
  }


  const handleChangeUser = (event) => setUser(event.target.value);

  const handleChange = (event) => setMsg(event.target.value);

  return (
    
    <div className="App">
      <label className="app-label">
        Username: <input type="text" name="name" onChange={handleChangeUser}/>
      </label>
      <label className="app-label">
        Message: <input type="text" onChange={handleChange}/>
        <button className="send-button" type="submit" onClick={handleClick}>Send</button>
      </label>
      <p>{resArr.map(e => 
        <p>{e}<br /></p>
      )}
      </p>
      <p>{'>'} {message} </p>
    </div>
  );
};

export default App;
