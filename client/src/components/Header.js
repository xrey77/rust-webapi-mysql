import React from "react";
import { Link } from 'react-router-dom';
import Login from "./Login";
import Registration from "./Registration";

function Header() {
    
   var username = sessionStorage.getItem("USERNAME");
   var userpic = "/users/" + sessionStorage.getItem("USERPIC");

  const logOut = (event) => {
    event.preventDefault();
    sessionStorage.removeItem("USERID");
    sessionStorage.removeItem("USERNAME");
    sessionStorage.removeItem("USERPIC");
    sessionStorage.removeItem("TOKEN");
    window.location = "/";
  }

    return(
<div className="container-fluid">
<nav class="navbar navbar-expand-lg bg-body-tertiary">
  <div class="container-fluid">
    <Link class="navbar-brand" to={'/'}><img className="logo" src="/logo192.png" alt="" /></Link>
    <button class="navbar-toggler" type="button" data-bs-toggle="offcanvas" href="#offcanvasExample" aria-controls="offcanvasExample">
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarSupportedContent">
      <ul class="navbar-nav me-auto mb-2 mb-lg-0">
        <li class="nav-item">
          <Link class="nav-link active" aria-current="page" to={'aboutus'}>About Us</Link>
        </li>
        <li class="nav-item dropdown">
          <a class="nav-link dropdown-toggle" href="#/" role="button" data-bs-toggle="dropdown" aria-expanded="false">
            Products
          </a>
          <ul class="dropdown-menu">
            <li><a class="dropdown-item" href="#/">Product 1</a></li>
            <li><a class="dropdown-item" href="#/">Product 2</a></li>
            <li><hr class="dropdown-divider"/></li>
            <li><a class="dropdown-item" href="#/">Product 3</a></li>
          </ul>
        </li>
        <li class="nav-item">
          <Link class="nav-link" to={'contactinfo'}>Contact Info</Link>
        </li>
      </ul>
      {/* USERS LOGIN AND REGISTRATION */}
      <ul class="navbar-nav mr-auto">
      {
        username !== null ?
          <li class="nav-item dropdown">
            <a class="nav-link dropdown-toggle" href="#/" role="button" data-bs-toggle="dropdown" aria-expanded="false">
              <img src={userpic} className="usericon" alt="" />&nbsp;{username}
            </a>
            <ul class="dropdown-menu">
              <li><a onClick={logOut} class="dropdown-item" href="#/">Logout</a></li>
              <li><Link class="dropdown-item" to={'profile'}>Profile</Link></li>
              <li><hr class="dropdown-divider"/></li>
              <li><a class="dropdown-item" href="#/">Messenger</a></li>
            </ul>
          </li>

        :
        <>
        <li class="nav-item">
          <a class="nav-link" href="#/" data-bs-toggle="modal" data-bs-target="#staticLoginBackdrop">Login</a>
        </li>
        <li class="nav-item">
          <a class="nav-link" href="#/" data-bs-toggle="modal" data-bs-target="#staticRegistrationBackdrop">Register</a>
        </li>
        </>
      }

    </ul>

    </div>
    <Login/>
    <Registration/>    
  </div>
</nav>

{/* DRAWER MENU */}
<div class="offcanvas offcanvas-end drawer" tabindex="-1" id="offcanvasExample" aria-labelledby="offcanvasExampleLabel">
  <div class="offcanvas-header bg-success">
    <h5 class="offcanvas-title text-white" id="offcanvasExampleLabel">Drawer Menu</h5>
    <button type="button" class="btn-close btn-close-white" data-bs-dismiss="offcanvas" aria-label="Close"></button>
  </div>
  <div class="offcanvas-body">
  <ul class="nav flex-column">
  <li class="nav-item" data-bs-dismiss="offcanvas">
    <Link class="nav-link active" aria-current="page" to={'aboutus'}>About Us</Link>
  </li>
 <hr/>
  <li class="nav-item dropdown">
          <a class="nav-link dropdown-toggle" href="#/" role="button" data-bs-toggle="dropdown" aria-expanded="false">
            Products
          </a>
          <ul class="dropdown-menu">
            <li data-bs-dismiss="offcanvas"><a class="dropdown-item" href="#/">Product 1</a></li>
            <li data-bs-dismiss="offcanvas"><a class="dropdown-item" href="#/">Product 2</a></li>
            <li><hr class="dropdown-divider"/></li>
            <li data-bs-dismiss="offcanvas"><a class="dropdown-item" href="#/">Product 3</a></li>
          </ul>
        </li>

        <hr/>
 
  <li class="nav-item" data-bs-dismiss="offcanvas">
    <Link class="nav-link" to={'contactinfo'}>Contact Info</Link>
  </li>
  <hr/>
 
  {
    username !== null ?
    <>
    <hr/>
    <li class="nav-item dropdown">
            <a class="nav-link dropdown-toggle" href="#/" role="button" data-bs-toggle="dropdown" aria-expanded="false">
              <img src={userpic} className="usericon" alt="" />&nbsp;{username}
            </a>
            <ul class="dropdown-menu">
              <li data-bs-dismiss="offcanvas"><a class="dropdown-item" href="#/">Product 1</a></li>
              <li data-bs-dismiss="offcanvas"><a class="dropdown-item" href="#/">Product 2</a></li>
              <li><hr class="dropdown-divider"/></li>
              <li data-bs-dismiss="offcanvas"><a class="dropdown-item" href="#/">Product 3</a></li>
            </ul>
          </li>  
          <hr/>
    </>      
    :
    <>
    <li class="nav-item" data-bs-dismiss="offcanvas">
    <a class="nav-link" href="#/" data-bs-toggle="modal" data-bs-target="#staticLoginBackdrop">Login</a>
  </li>
  <hr/>
 
  <li class="nav-item" data-bs-dismiss="offcanvas">
    <a class="nav-link" href="#/" data-bs-toggle="modal" data-bs-target="#staticRegistrationBackdrop">Registration</a>
  </li>
  <hr/>
  </>
  }
   
</ul>


  </div>
</div>
</div>
    );
}

export default Header;