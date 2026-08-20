const { invoke } = window.__TAURI__?.core || {};
const call = (command, args) => invoke ? invoke(command, args) : Promise.reject(new Error('Tauri runtime is unavailable.'));

const money = (value) => `ETB ${Number(value || 0).toLocaleString(undefined,{minimumFractionDigits:2,maximumFractionDigits:2})}`;
const esc = (value) => String(value ?? '').replace(/[&<>\"]/g, (c) => ({'&':'&amp;','<':'&lt;','>':'&gt;','\"':'&quot;'}[c]));

export async function renderItems(root, onBack) {
  root.innerHTML = `<div class="app"><aside class="sidebar"><div class="logo">NATRA</div><nav class="nav"><button type="button" data-back>Dashboard</button><button type="button" class="active">Items & Categories</button><button type="button" data-waiters>Waiters</button></nav><button id="logout" class="logout" type="button">Sign out</button></aside><main class="main"><header class="topbar"><div><h1 class="title">Items & Categories</h1><p class="subtitle">Separate cookable foods from ready-made products.</p></div><span class="status">● Local mode</span></header><div id="items-error" class="error" role="alert"></div><section class="split-panels"><div class="panel"><div class="panel-head"><div><h2>Categories</h2><p>Choose whether a category is cookable or ready-made.</p></div></div><form id="category-form" class="compact-form"><input id="category-name" placeholder="Category name" required><select id="category-type"><option value="COOKABLE">Cookable</option><option value="READY_MADE">Ready-made</option></select><button class="primary" type="submit">Add category</button></form><div id="category-list" class="stack-list"></div></div><div class="panel"><div class="panel-head"><div><h2>Add item</h2><p>Cost is required only for ready-made items.</p></div></div><form id="item-form" class="compact-form"><input id="item-name" placeholder="Item name" required><select id="item-category" required></select><div class="form-grid"><input id="item-price" type="number" min="0" step="0.01" placeholder="Selling price" required><input id="item-qty" type="number" min="0" step="0.01" placeholder="Quantity" required></div><div id="cost-wrap"><input id="item-cost" type="number" min="0" step="0.01" placeholder="Purchase cost"></div><button class="primary" type="submit">Add item</button></form></div></section><section class="panel"><div class="panel-head"><div><h2>Active items</h2><p>Ready-made items carry direct purchase cost; cookable items do not.</p></div><div class="filters"><button type="button" class="filter active" data-filter="ALL">All</button><button type="button" class="filter" data-filter="COOKABLE">Cookable</button><button type="button" class="filter" data-filter="READY_MADE">Ready-made</button></div></div><div id="items-list" class="item-grid"></div></section></main></div>`;

  const errorBox = root.querySelector('#items-error');
  const showError = (e) => { errorBox.textContent = String(e?.message || e || 'Operation failed'); };
  const categorySelect = root.querySelector('#item-category');
  const costWrap = root.querySelector('#cost-wrap');
  let categories = [];
  let items = [];
  let filter = 'ALL';

  const load = async () => {
    try { categories = await call('list_categories'); items = await call('list_items',{itemType:null}); renderCategories(); renderCategoryOptions(); renderItems(); } catch (e) { showError(e); }
  };
  const renderCategories = () => {
    root.querySelector('#category-list').innerHTML = categories.length ? categories.map(c => `<div class="list-row"><div><strong>${esc(c.name)}</strong><span class="tag">${c.item_type === 'COOKABLE' ? 'Cookable' : 'Ready-made'}</span></div><button type="button" class="danger-text" data-remove-category="${c.id}">Remove</button></div>`).join('') : '<div class="empty">No categories yet.</div>';
    root.querySelectorAll('[data-remove-category]').forEach(b => b.onclick = async () => { try { await call('remove_category',{id:Number(b.dataset.removeCategory)}); await load(); } catch(e) { showError(e); } });
  };
  const renderCategoryOptions = () => { categorySelect.innerHTML = categories.length ? categories.map(c => `<option value="${c.id}">${esc(c.name)} · ${c.item_type === 'COOKABLE' ? 'Cookable' : 'Ready-made'}</option>`).join('') : '<option value="">Add a category first</option>'; updateCostVisibility(); };
  const updateCostVisibility = () => { const c = categories.find(x => x.id === Number(categorySelect.value)); costWrap.style.display = c?.item_type === 'READY_MADE' ? 'block' : 'none'; };
  const renderItems = () => { const visible = filter === 'ALL' ? items : items.filter(i => i.item_type === filter); root.querySelector('#items-list').innerHTML = visible.length ? visible.map(i => `<article class="item-card"><div class="item-card-top"><span class="tag">${i.item_type === 'COOKABLE' ? 'Cookable' : 'Ready-made'}</span><button type="button" class="danger-text" data-remove-item="${i.id}">Remove</button></div><h3>${esc(i.name)}</h3><p>${esc(i.category_name)}</p><div class="item-meta"><span>Sell<br><strong>${money(i.selling_price)}</strong></span><span>Qty<br><strong>${Number(i.quantity).toLocaleString()}</strong></span>${i.item_type === 'READY_MADE' ? `<span>Cost<br><strong>${money(i.purchase_cost)}</strong></span>` : '<span>Cost<br><strong>Raw materials</strong></span>'}</div></article>`).join('') : '<div class="empty">No items in this filter.</div>'; root.querySelectorAll('[data-remove-item]').forEach(b => b.onclick = async () => { try { await call('remove_item',{id:Number(b.dataset.removeItem)}); await load(); } catch(e) { showError(e); } }); };

  categorySelect.onchange = updateCostVisibility;
  root.querySelectorAll('[data-filter]').forEach(b => b.onclick = () => { filter=b.dataset.filter; root.querySelectorAll('[data-filter]').forEach(x=>x.classList.toggle('active',x===b)); renderItems(); });
  root.querySelector('#category-form').onsubmit = async (e) => { e.preventDefault(); try { await call('add_category',{input:{name:root.querySelector('#category-name').value.trim(),itemType:root.querySelector('#category-type').value}}); e.target.reset(); await load(); } catch(err) { showError(err); } };
  root.querySelector('#item-form').onsubmit = async (e) => { e.preventDefault(); const categoryId=Number(categorySelect.value); const category=categories.find(c=>c.id===categoryId); const cost=category?.item_type==='READY_MADE' ? Number(root.querySelector('#item-cost').value) : null; try { await call('add_item',{input:{categoryId,name:root.querySelector('#item-name').value.trim(),purchaseCost:cost,sellingPrice:Number(root.querySelector('#item-price').value),quantity:Number(root.querySelector('#item-qty').value)}}); e.target.reset(); await load(); } catch(err) { showError(err); } };
  root.querySelector('[data-back]').onclick = onBack;
  await load();
}
