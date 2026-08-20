import './app.css';
import { endSession, isAuthenticated, startSession } from './auth/session.js';
import { renderWaiters } from './pages/waiters.js';
import { renderItems } from './pages/items.js';

const { invoke } = window.__TAURI__?.core || {};
const call = (command, args) => invoke ? invoke(command, args) : Promise.reject(new Error('Tauri runtime is unavailable.'));
const nav = ['Dashboard','POS','Waiters','Items & Categories','Raw Materials','Expenses','Reports','Settings'];
const money = (value) => `ETB ${Number(value || 0).toLocaleString(undefined,{minimumFractionDigits:2,maximumFractionDigits:2})}`;
const setError = (message) => { const el=document.querySelector('#err'); if(el) el.textContent=message; };

async function start() {
  try { const initialized=await call('app_initialized'); if(!initialized) return setup(); if(isAuthenticated()) return dashboard(); login(); }
  catch(error) { console.error(error); setup(); }
}
function setup() {
  document.querySelector('#app').innerHTML=`<main class="login"><section class="login-box"><div class="brand-mark">N</div><h1>NATRA</h1><p>Restaurant Management · First-time setup</p><form id="setup"><label>Full name</label><input id="name" required><label>Username</label><input id="user" minlength="3" required><label>Password</label><input id="pass" type="password" minlength="8" required><label>Confirm password</label><input id="confirm" type="password" minlength="8" required><button class="primary" type="submit">Create Administrator</button><div id="err" class="error" role="alert"></div></form></section></main>`;
  document.querySelector('#setup').onsubmit=async(e)=>{e.preventDefault();const name=document.querySelector('#name').value.trim(),username=document.querySelector('#user').value.trim(),password=document.querySelector('#pass').value,confirmation=document.querySelector('#confirm').value;if(!name)return setError('Full name is required.');if(password!==confirmation)return setError('Passwords do not match.');try{await call('create_admin',{username,password,fullName:name});startSession();dashboard();}catch(error){setError(String(error));}};
}
function login() {
  document.querySelector('#app').innerHTML=`<main class="login"><section class="login-box"><div class="brand-mark">N</div><h1>NATRA</h1><p>Restaurant Management</p><form id="login"><label>Username</label><input id="user" required><label>Password</label><input id="pass" type="password" required><button class="primary" type="submit">Sign in</button><div id="err" class="error" role="alert"></div></form></section></main>`;
  document.querySelector('#login').onsubmit=async(e)=>{e.preventDefault();try{const valid=await call('login',{username:document.querySelector('#user').value.trim(),password:document.querySelector('#pass').value});if(!valid)return setError('Invalid username or password.');startSession();dashboard();}catch(error){setError(String(error));}};
}
async function dashboard() {
  let summary={}; try{summary=await call('dashboard_summary');}catch(e){console.error(e);}
  document.querySelector('#app').innerHTML=`<div class="app"><aside class="sidebar"><div class="logo">NATRA</div><nav class="nav">${nav.map(item=>`<button data-nav="${item}" class="${item==='Dashboard'?'active':''}" type="button">${item}</button>`).join('')}</nav><button id="logout" class="logout" type="button">Sign out</button></aside><main class="main"><header class="topbar"><h1 class="title">Dashboard</h1><span class="status">● Local mode</span></header><section class="cards"><div class="card"><div class="label">Total Sales</div><div class="value">${money(summary.sales)}</div></div><div class="card"><div class="label">Ready-made Cost</div><div class="value">${money(summary.ready_made_cost)}</div></div><div class="card"><div class="label">Raw Material Cost</div><div class="value">${money(summary.raw_material_cost)}</div></div><div class="card"><div class="label">Net Profit</div><div class="value">${money(summary.net_profit)}</div></div></section><section class="panel"><h2>Restaurant Management</h2><p>Use the sidebar to manage waiters and products. Each module is connected to the local database.</p></section></main></div>`;
  rootNav();
}
function rootNav(){
  document.querySelectorAll('[data-nav]').forEach(button=>{button.onclick=()=>{const page=button.dataset.nav;if(page==='Dashboard')dashboard();else if(page==='Waiters')renderWaiters(document.querySelector('#app'),dashboard);else if(page==='Items & Categories')renderItems(document.querySelector('#app'),dashboard);}});
  const logout=document.querySelector('#logout'); if(logout)logout.onclick=()=>{endSession();login();};
}
window.addEventListener('natra:logout',()=>{endSession();login();});
start();
