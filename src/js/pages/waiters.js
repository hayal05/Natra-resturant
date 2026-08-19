const { invoke } = window.__TAURI__?.core || {};
const call = (command, args) =>
  invoke ? invoke(command, args) : Promise.reject(new Error('Tauri runtime is unavailable.'));

const money = (value) => `ETB ${Number(value || 0).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;

export async function renderWaiters(app, onBack) {
  let waiters = [];
  let error = '';

  try {
    waiters = await call('list_waiters');
  } catch (err) {
    error = String(err);
  }

  const draw = () => {
    app.querySelector('#waiter-list').innerHTML = waiters.length
      ? waiters.map((waiter) => `
          <article class="waiter-row">
            <div class="waiter-avatar">${escapeHtml(waiter.name.charAt(0).toUpperCase())}</div>
            <div class="waiter-info">
              <strong>${escapeHtml(waiter.name)}</strong>
              <span>Active waiter</span>
            </div>
            <div class="waiter-money">
              <span>Today's receivable</span>
              <strong>${money(waiter.today_receivable)}</strong>
            </div>
            <button class="danger ghost" data-remove="${waiter.id}" type="button">Remove</button>
          </article>`).join('')
      : '<div class="empty">No active waiters yet. Add the first waiter above.</div>';

    app.querySelectorAll('[data-remove]').forEach((button) => {
      button.onclick = async () => {
        const waiter = waiters.find((item) => item.id === Number(button.dataset.remove));
        if (!waiter || !confirm(`Remove ${waiter.name}? Historical sales will be preserved.`)) return;
        try {
          await call('remove_waiter', { id: waiter.id });
          waiters = await call('list_waiters');
          draw();
        } catch (err) {
          app.querySelector('#waiter-error').textContent = String(err);
        }
      };
    });
  };

  app.innerHTML = `
    <div class="app">
      <aside class="sidebar">
        <div class="logo">NATRA</div>
        <nav class="nav"><button id="back-dashboard" type="button">Dashboard</button><button class="active" type="button">Waiters</button></nav>
        <button id="waiter-logout" class="logout" type="button">Sign out</button>
      </aside>
      <main class="main">
        <header class="topbar"><div><h1 class="title">Waiters</h1><p class="page-subtitle">Manage active waiters and their daily receivables.</p></div><span class="status">● Local mode</span></header>
        <section class="panel waiter-form-panel">
          <div><h2>Add waiter</h2><p>New waiters will automatically appear in POS.</p></div>
          <form id="waiter-form" class="inline-form">
            <input id="waiter-name" maxlength="100" placeholder="Waiter name" autocomplete="off" required>
            <button class="primary compact" type="submit">Add waiter</button>
          </form>
          <div id="waiter-error" class="error">${escapeHtml(error)}</div>
        </section>
        <section class="panel">
          <div class="section-heading"><div><h2>Active waiters</h2><p>Today's receivable is calculated from recorded sales.</p></div><span class="count">${waiters.length}</span></div>
          <div id="waiter-list" class="waiter-list"></div>
        </section>
      </main>
    </div>`;

  app.querySelector('#back-dashboard').onclick = onBack;
  app.querySelector('#waiter-form').onsubmit = async (event) => {
    event.preventDefault();
    const input = app.querySelector('#waiter-name');
    const name = input.value.trim();
    if (!name) return;
    try {
      await call('add_waiter', { name });
      waiters = await call('list_waiters');
      input.value = '';
      app.querySelector('#waiter-error').textContent = '';
      draw();
      input.focus();
    } catch (err) {
      app.querySelector('#waiter-error').textContent = String(err);
    }
  };

  app.querySelector('#waiter-logout').onclick = () => {
    window.dispatchEvent(new CustomEvent('natra:logout'));
  };

  draw();
}

function escapeHtml(value) {
  return String(value).replace(/[&<>'"]/g, (char) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', "'": '&#39;', '"': '&quot;' }[char]));
}
