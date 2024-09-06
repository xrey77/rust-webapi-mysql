import React, { useEffect, useState } from "react";
import $ from 'jquery';
import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:8080",
  headers: {'Accept': 'application/json',
            'Content-Type': 'application/json',
            'Authorization': 'inherit'},
})

function Profile() {

    var uid = sessionStorage.getItem("USERID");
    var token = sessionStorage.getItem("TOKEN");
    var userpic = "/users/" + sessionStorage.getItem("USERPIC");
    const [firstname, setfirstname] = useState('');
    const [lastname, setlastname] = useState('');
    const [emailadd, setemailadd] =useState('');
    const [mobileno, setmobileno] = useState('');
    const [newpassword, setnewpassword] = useState('');
    // const [selectedFile, setSelectedifle] = useState('');
    const [confirmnewpassword, setconfirmnewpassword] = useState('');
    const [changepassword, setchangepassword] = useState(false);
    const [onetimepassword, setonetimepassword] = useState(false);
    const [message, setmessage] = useState('');

    useEffect(() => {

      const getUserid = () => {
          api.get(`/users/${uid}`,{headers: {
              Authorization: `Bearer ${token}`
          }})
          .then((res) => {
            if (res.data.statuscode === 201) {
              setfirstname(res.data.firstname);
              setlastname(res.data.lastname);
              setemailadd(res.data.emailadd);
              setmobileno(res.data.mobileno);
            } else {
              setmessage(res.data.message);
            }
          },(error) => {
              setmessage(error.message);
          });

      }
      getUserid();
    },[uid, token])


    const changePix = (event) => {
      event.preventDefault();
      $("#userx").attr('src',URL.createObjectURL(event.target.files[0]));     
      const data = new FormData()
      data.append('file', event.target.files[0])
      api.patch(`/users/updateuserpic/${uid}`, data,{headers: {      
      "Content-Type": "Multipart/form-data",
      }})
        .then((response) => {
            setmessage(response.data.message);
            window.setTimeout(() => {
              setmessage('');
            }, 6000);
      },(error) => {
          setmessage(error.message);
        });
    }

    const updatePassword = (event) => {
      event.preventDefault()
      if (newpassword !== confirmnewpassword) {
        setmessage("New password does not matched....");
        return;
      }
      const data = JSON.stringify({ password: newpassword });
      api.patch(`/users/updatepwd/${uid}`, data,{headers: {
          Authorization: `Bearer ${token}`
      }})
      .then((res) => {
        if (res.data.statuscode === 201) {
          setmessage(res.data.message);          

        } else {
          setmessage(res.data.message);          
        }
      },(error) => {
          setmessage(error.message);
      });
      window.setTimeout(() => {
        setmessage('');
      }, 5000);
    }

    const changepassword_mouseUp = (event) => {
      event.preventDefault();
      setmessage("please double click to toggle change password.")
    }

    const onepassword_mouseUp = (event) => {
      event.preventDefault();
      setmessage("please double click to toggle One Time Password.")
    }

    const mouseLeave = (event) => {
        event.preventDefault();
        setmessage('');
    }
    const change_pwd = (event) => {
      event.preventDefault();
      if ($('#changePasswordChecked').is(":checked")) {
        setchangepassword(true);
        setonetimepassword(false);
        $('#onetimePasswordChecked').prop('checked', false);        
        $('#changePasswordChecked').prop('checked', true);        
      } else {
        setchangepassword(false);
        $('#changePasswordChecked').prop('checked', false);
      }
    }

    const onetime_pwd = (event) => {
      event.preventDefault();
      if ($('#onetimePasswordChecked').is(":checked")) {
        setchangepassword(false);
        setonetimepassword(true);        
        $('#changePasswordChecked').prop('checked', false);
        $('#onetimePasswordChecked').prop('checked', true);        
      } else {
        setonetimepassword(false);
        $('#onetimePasswordChecked').prop('checked', false);
      }

    }

    const updateProfile = (event) => {
      event.preventDefault();
      const data = JSON.stringify({ firstname: firstname, lastname: lastname, mobileno: mobileno  });
      api.patch(`/users/${uid}`, data,{headers: {
          Authorization: `Bearer ${token}`
      }})
      .then((res) => {
        if (res.data.statuscode === 201) {
          setmessage(res.data.message);          

        } else {
          setmessage(res.data.message);          
        }
          console.log(res.data);

      },(error) => {
          setmessage(error.message);
      });

    }

    return (
    <div className="card profile-width mt-1">
      <div className="card-header">
        <h3>My Profile</h3>
      </div>
      <div class="card-body">
      {/* enctype="multipart/form-data"  */}
         <form onSubmit={updateProfile} autoComplete="false">
          <div className="row">
            <div className="col">
              <div class="mb-3">
                <input type="text" value={firstname} onChange={e => setfirstname(e.target.value)} class="form-control form-control-sm" id="fname" required placeholder="Firstname"/>
              </div>            
              <div class="mb-3">
                <input type="text" value={lastname} onChange={e => setlastname(e.target.value)} class="form-control form-control-sm" id="lname" required placeholder="Lastname"/>
              </div>            
            </div>
            <div className="col">

                <div className="row">
                    <div className="col">
                      {
                        userpic !== null ?
                          <img id="userx" className="userpic" src={userpic} alt="" />
                        :
                          <img id="userx" className="userpic" src="/userx.png" alt="" />
                      }
                    </div>
                    <div className="col">
                      <div class="mb-3">
                        <input onChange={changePix} class="form-control form-control-sm" id="file" type="file"/>
                      </div>
                    </div>
                </div>
                  
            </div>
          </div>

          <div className="row">
            <div className="col">
              <div class="mb-3">
                <input type="email" value={emailadd} onChange={e => setemailadd(e.target.value)} class="form-control form-control-sm" id="emailadd" readOnly placeholder="Email Address"/>
              </div>            
            </div>

            <div className="col">
               <div class="mb-3">
                <input type="text" value={mobileno} onChange={e => setmobileno(e.target.value)} class="form-control form-control-sm" id="mobileno" required placeholder="Mobile No."/>
               </div>            
            </div>
            <div class="mb-3">
              <button id="saveprofile" type="submit" className="btn btn-primary">update profile</button>
            </div>
          </div>
         <hr/>
          {/* RADIO BUTTON SWITCH */}
          <div className="row">
              <div className="col">

                <div class="form-check form-switch" >
                  <input onChange={change_pwd} onMouseEnter={changepassword_mouseUp} onMouseLeave={mouseLeave} class="form-check-input" type="checkbox" role="switch" id="changePasswordChecked"/>
                  <label class="form-check-label" htmlFor="changePasswordChecked" onMouseEnter={changepassword_mouseUp} onMouseLeave={mouseLeave}>Change Password</label>
                </div>
                {
                  changepassword === true ?
                  <>
                  <div class="mb-3">
                    <input type="text" value={newpassword} onChange={e => setnewpassword(e.target.value)} class="form-control form-control-sm" id="newpwd1" 
                    required placeholder="enter New Password"
                    />
                  </div>            
                  <div class="mb-3">
                    <input type="text" value={confirmnewpassword} onChange={e => setconfirmnewpassword(e.target.value)} class="form-control form-control-sm" id="newpwd2" required placeholder="confirm New Password"/>
                  </div>     
                  <button onClick={updatePassword} type="button" className="btn btn-primary">change password</button>
                  </>
                  :
                  null
                }


              </div>
              <div className="col">

                <div class="form-check form-switch">
                  <input onChange={onetime_pwd} onMouseEnter={onepassword_mouseUp} onMouseLeave={mouseLeave} class="form-check-input" type="checkbox" role="switch" id="onetimePasswordChecked"/>
                  <label class="form-check-label" htmlFor="onetimePasswordChecked" onMouseEnter={onepassword_mouseUp} onMouseLeave={mouseLeave}>Onetime Password</label>
                </div>
                {
                  onetimepassword === true ?
                    <>
                    <div className="w-100 text-center">
                      Install <strong>Google Authenticator</strong> or <strong>Microsoft Authenticator</strong>
                      &nbsp;in your <strong>Mobile Phone</strong>, and Scan <strong>QR CODE</strong> below.
                      <img src="/qrcode.png" className="xqr-code" alt="qrcode" />
                      </div>
                      


                    </>
                  :
                  null
                }

              </div>
          </div>

          </form>        
      </div>
      <card-footer>
        <hr/>
        <div className="w-100 cardfooter-size text-danger text-center">{message}</div>
      </card-footer>
    </div>      
    );
}

export default Profile;