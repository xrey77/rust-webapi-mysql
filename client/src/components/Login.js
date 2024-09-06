import React, { useState } from "react";
import axios from 'axios';

function Login() {
    const [usrname, setusrname] = useState('')
    const [pword, setpword] = useState('');
    const [message, setmessage] = useState('');

    const api = axios.create({
        baseURL: "http://localhost:8080",
        headers: {'Accept': 'application/json',
                  'Content-Type': 'application/json',
                  'Authorization': 'inherit'},
    })
    

    const submitData = (event) => {
        event.preventDefault();
        const data =JSON.stringify({ username: usrname, password: pword });
        api.post("/auth/login", data)
           .then((res) => {
            setmessage(res.data.message);                            
            if (res.data.statuscode === 201) {
                sessionStorage.setItem('USERID', res.data.user.id);            
                sessionStorage.setItem('USERNAME', res.data.user.username);            
                sessionStorage.setItem('USERPIC',res.data.user.profilepic);    
                sessionStorage.setItem('TOKEN',res.data.token);    
                window.location = "/";
            } 
            window.setTimeout(() => {
                setmessage('');                            
            }, 3000);
            
        }, (error) => {
            setmessage(error.message);
            window.setTimeout(() => {
                setmessage('');                            
            }, 3000);
            console.log(error.message);
            return;
        });




    }

    const clearData = (event) => {
        event.preventDefault();
        setusrname('');
        setpword('');
        console.log("data cleared...");
    }

    return(
<div class="modal fade" id="staticLoginBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticLoginBackdropLabel" aria-hidden="true">
  <div class="modal-dialog modal-sm modal-dialog-centered">
    <div class="modal-content">
      <div class="modal-header bg-primary">
        <h1 class="modal-title fs-5 text-white" id="staticLoginBackdropLabel">User's Login</h1>
        <button onClick={clearData} type="button" class="btn-close btn-close-white" data-bs-dismiss="modal" aria-label="Close"></button>
      </div>
      <div class="modal-body">
        <form onSubmit={submitData} autoComplete="false">
        <div class="mb-3">
            <input type="text" value={usrname} onChange={e => setusrname(e.target.value)} class="form-control" id="usrname" required placeholder="enter Username"/>
        </div>            
        <div class="mb-3">
            <input type="password" value={pword} onChange={e => setpword(e.target.value)} class="form-control" id="pword" required placeholder="enter Password"/>
        </div>            
        <button type="submit" class="btn btn-primary">login</button>

        </form>
      </div>
      <div class="modal-footer">
         <div className="w-100 text-danger fontsize-10">{message}</div>
      </div>
    </div>
  </div>
</div>
    );
}

export default Login;
