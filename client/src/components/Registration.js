import React, { useState } from "react";
import axios from 'axios';

function Registration() {
  const [firstname, setfirstname] = useState('');
  const [lastname, setlastname] = useState('');
  const [emailadd, setemailadd] = useState('');
  const [mobileno, setmobileno] = useState('');
  const [username, setusername] = useState('');
  const [password, setpassword] = useState('');
  const [message, setmessage] = useState('');

  const api = axios.create({
    baseURL: "http://localhost:8080",
    headers: {'Accept': 'application/json',
              'Content-Type': 'application/json',
              'Authorization': 'inherit'},
  })

  const clearData = () => {
    setfirstname('');
    setlastname('');
    setemailadd('');
    setmobileno('');
    setusername('');
    setpassword('');
  }

  const submitRegistration = (event) => {
    event.preventDefault();
    const data =JSON.stringify({ 
      firstname: firstname, lastname: lastname,
      emailadd: emailadd, mobileno: mobileno,
      username: username, password: password });
    api.post("/auth/register", data)
       .then((res) => {
        setmessage(res.data.message);                            
        if (res.data.statuscode === 201) {
            clearData();
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
        return;
    });

  }

    return (
<div class="modal fade" id="staticRegistrationBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticRegistrationBackdropLabel" aria-hidden="true">
  <div class="modal-dialog modal-dialog-centered">
    <div class="modal-content">
      <div class="modal-header bg-success">
        <h1 class="modal-title fs-5 text-white" id="staticRegistrationBackdropLabel">Registration</h1>
        <button onClick={clearData} type="button" class="btn-close btn-close-white" data-bs-dismiss="modal" aria-label="Close"></button>
      </div>
      <div class="modal-body">
        <form onSubmit={submitRegistration} autoComplete="false">
            <div className="row">
                <div className="col">
                  <div class="mb-3">
                    <input type="text" value={firstname} onChange={e => setfirstname(e.target.value)} class="form-control" id="fname" required placeholder="enter Firstname"/>
                  </div>            
                </div>
                <div className="col">
                  <div class="mb-3">
                    <input type="text" value={lastname} onChange={e => setlastname(e.target.value)} class="form-control" id="lname" required placeholder="enter Lastname"/>
                  </div>            
                </div>
            </div>

            <div className="row">
                <div className="col">
                  <div class="mb-3">
                    <input type="email" value={emailadd} onChange={e => setemailadd(e.target.value)} class="form-control" id="emailadd" required placeholder="enter Email Address"/>
                  </div>            
                </div>
                <div className="col">
                  <div class="mb-3">
                    <input type="text" value={mobileno} onChange={e => setmobileno(e.target.value)} class="form-control" id="mobileno" required placeholder="enter Mobile No."/>
                  </div>            
                </div>
            </div>

            <div className="row">
                <div className="col">
                  <div class="mb-3">
                    <input type="text" value={username} onChange={e => setusername(e.target.value)} class="form-control" id="uname" required placeholder="enter Username"/>
                  </div>            
                </div>
                <div className="col">
                  <div class="mb-3">
                    <input type="password" value={password} onChange={e => setpassword(e.target.value)} class="form-control" id="pword" required placeholder="enter Password"/>
                  </div>            
                </div>
            </div>

            <button type="submit" class="btn btn-success">register</button>

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

export default Registration;