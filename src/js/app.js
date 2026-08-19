import './app.css';
import { endSession, isAuthenticated, startSession } from './auth/session.js';
import { renderWaiters } from './pages/waiters.js';

const { invoke } = window.__TAURI__?.core || {};
const call = (command, args) =>
  invoke ? invoke(command, args) : Promise.reject(new Error('Tauri runtime is unavailable.'));

const nav = ['Dashboard', 'POS', 'Waiters', 'Items & Categories', 'Raw Materials', 'Expenses', 'Reports', 'Settings'];

function money(value) {
  return `ETB ${Number(value || 0).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
}

function setError(message) {
  const error = document.querySelector('#err');
  if (error) error.textContent = message;
}

async function start() {
  try {
    const initialized = await call('app_initialized');
    if (!initialized) {
      setup();
      return;
    }
    if (isAuthenticated()) dashboard();
    else login();
  } catch (error) {
    console.error(error);
    setup();
  }
}

function setup() {
  document.querySelector('#app').innerHTML = `
    <main class="login"><section class="login-box">
      <div class="brand-mark">N</div><h1>NATRA</h1><p>Restaurant Management · First-time setup</p>
      <form id="setup">
        <label for="name">Full name</label><input id="name" autocomplete="name" required>
        <label for="user">Username</label><input id="user" minlength="3" autocomplete="username" required>
        <label for="pass">Password</label><input id="pass" type="password" minlength="8" autocomplete="new-password" required>
        <label for="confirm">Confirm password</label><input id="confirm" type="password" minlength="8" autocomplete="new-password" required>
        <button class="primary" type="submit">Create Administrator</button><div id="err" class="error" role="alert"></div>
      </form>
    </section></main>`;

  document.querySelector('#setup').onsubmit = async (event) => {
    event.preventDefault();
    const name = document.querySelector('#name').value.trim();
    const username = document.querySelector('#user').value.trim();
    const password = document.querySelector('#pass').value;
    const confirmation = document.querySelector('#confirm').value;
    if (!name) return setError('Full name is required.');
    if (password !== confirmation) return setError('Passwords do not match.');
    try {
      await call('create_admin', { username, password, fullName: name });
      startSession();
      dashboard();
    } catch (error) { setError(String(error)); }
  };
}

function login() {
  document.querySelector('#app').innerHTML = `
    <main class="login"><section class="login-box">
      <div class="brand-mark">N</div><h1>NATRA</h1><p>Restaurant Management</p>
      <form id="login">
        <label for="user">Username</label><input id="user" autocomplete="username" required>
        <label for="pass">Password</label><input id="pass" type="password" autocomplete="current-password" required>
        <button class="primary" type="submit">Sign in</button><div id="err" class="error" role="alert"></div>
      </form>
    </section></main>`;

  document.querySelector('#login').onsubmit = async (event) => {
    event.preventDefault();
    const username = document.querySelector('#user').value.trim();
    const password = document.querySelector('#pass').value;
    try {
      const valid = await call('login', { username, password });
      if (!valid) return setError('Invalid username or password.');
      startSession();
      dashboard();
    } catch (error) { setError(String(error)); }
  };
}

async function dashboard() {
  let summary = {};
  try { summary = await call('dashboard_summary'); } catch (error) { console.error(error); }

  document.querySelector('#app').innerHTML = `
    <div class="app">
      <aside class="sidebar">
        <div class="logo">NATRA</div>
        <nav class="nav">
          ${nav.map((item) => `<button data-nav="${item}" class="${item === 'Dashboard' ? 'active' : ''}" type="button">${item}</button>`).join('')}
        </nav>
        <button id="logout" class="logout" type="button">Sign out</button>
      </aside>
      <main class="main">
        <header class="topbar"><h1 class="title">Dashboard</h1><span class="status">● Local mode</span></header>
        <section class="cards">
          <div class="card"><div class="label">Total Sales</div><div class="value">${money(summary.sales)}</div></div>
          <div class="card"><div class="label">Ready-made Cost</div><div class="value">${money(summary.ready_made_cost)}</div></div>
          <div class="card"><div class="label">Raw Material Cost</div><div class="value">${money(summary.raw_material_cost)}</div></div>
          <div class="card"><div class="label">Net Profit</div><div class="value">${money(summary.net_profit)}</div></div>
        </section>
        <section class="panel"><h2>Restaurant Management</h2><p>Use the Waiters section to add staff and manage daily receivables. More modules will be added one step at a time.</p></section>
      </main>
    </div>`;

  document.querySelectorAll('[data-nav]').forEach((button) => {
    button.onclick = () => {
      if (button.dataset.nav === 'Waiters') renderWaiters(document.querySelector('#app'), dashboard);
    };
  });
  document.querySelector('#logout').onclick = () => { endSession(); login(); };
}

window.addEventListener('natra:logout', () => { endSession(); login(); });
start();
