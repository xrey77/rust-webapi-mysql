import { Routes, Route } from "react-router-dom";
import './App.css';
import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap/dist/js/bootstrap.bundle';
import '@popperjs/core/dist/umd/popper';
import '@popperjs/core/dist/umd/popper.min.js';

import Home from "./components/Home";
import Aboutus from "./components/About";
import Contactinfo from "./components/Contactinfo";
import Profile from './components/Profile';

function App() {
  return (
    <Routes>
      <Route path='/' element={<Home/>} />
      <Route path='/aboutus' element={<Aboutus />} />
      <Route path='/contactinfo' element={<Contactinfo/>} />
      <Route path='/profile' element={<Profile />} />
      {/* 
      <Route path='/viewcart' element={<ViewCart/>} />
      <Route path='/peritem' element={<Items />} />
      <Route path='/sale' element={<Sale />}/> */}
    </Routes>
    );
}

export default App;
