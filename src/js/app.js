import '../css/main.css';

const pages = {
  dashboard: { title: 'Dashboard', body: dashboard() },
  pos: { title: 'POS', body: placeholder('Point of sale workspace') },
  waiters: { title: 'Waiters', body: placeholder('Waiter management workspace') },
  items: { title: 'Items & Categories', body: placeholder('Items and categories workspace') },
  raw: { title: 'Raw Materials', body: placeholder('Raw material purchases and usage workspace') },
  expenses: { title: 'Expenses', body: placeholder('Other expenses workspace') },
  reports: { title: 'Reports', body: placeholder('Monthly revenue, costs and cash flow reports') },
  settings: { title: 'Settings', body: placeholder('Application and optional Turso sync settings') }
};

function dashboard() {
  return `
    <div class="grid stats">
      ${stat('Total Sales', 'ETB 0')}
      ${stat('Ready-Made Cost', 'ETB 0')}
      ${stat('Raw Material Cost', 'ETB 0')}
      ${stat('Net Profit', 'ETB 0')}
    </div>
    <div class="grid" style="grid-template-columns:1fr 1fr; margin-top:18px">
      <section class="card"><h2 class="section-title">Waiter Receivables</h2><div class="placeholder">Waiter cards will appear here</div></section>
      <section class="card"><h2 class="section-title">Top Sales Products</h2><div class="placeholder">Top products will appear here</div></section>
      <section class="card"><h2 class="section-title">Sales Mix</h2><div class="placeholder">Daily / Weekly / Monthly donut chart</div></section>
      <section class="card"><h2 class="section-title">Cash Flow</h2><div class="placeholder">Daily / Weekly / Monthly line chart</div></section>
    </div>`;
}

function stat(label, value) { return `<div class="card"><div class="stat-label">${label}</div><div class="stat-value">${value}</div></div>`; }
function placeholder(text) { return `<div class="card"><div class="placeholder">${text}</div></div>`; }

const icons = { dashboard:'▦', pos:'▣', waiters:'♙', items:'◈', raw:'◇', expenses:'◌', reports:'▥', settings:'⚙' };

function render(page = 'dashboard') {
  const current = pages[page] || pages.dashboard;
  document.querySelector('#app').innerHTML = `
    <div class="app-shell">
      <aside class="sidebar">
        <div class="brand"><h2 class="brand-title">NATRA</h2><div class="brand-subtitle">RESTAURANT MANAGEMENT</div></div>
        <nav class="nav">${Object.keys(pages).map(key => `<button class="${key===page?'active':''}" data-page="${key}">${icons[key]} &nbsp; ${pages[key].title}</button>`).join('')}</nav>
        <div class="sidebar-footer">Local-first • Optional Turso Sync</div>
      </aside>
      <main class="main">
        <header class="topbar"><h1>${current.title}</h1><span class="status">● Local Mode</span></header>
        <section class="content">${current.body}</section>
      </main>
    </div>`;
  document.querySelectorAll('[data-page]').forEach(button => button.addEventListener('click', () => render(button.dataset.page)));
}

render();
