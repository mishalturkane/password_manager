const API = '/api/passwords';

// ── Load all passwords ──────────────────────────────
async function loadAll() {
  document.getElementById('search-input').value = '';
  const res  = await fetch(API);
  const data = await res.json();
  renderList(data);
}

// ── Search passwords ────────────────────────────────
async function searchPasswords() {
  const q = document.getElementById('search-input').value.trim();
  if (!q) return loadAll();

  const res  = await fetch(`${API}/search?q=${encodeURIComponent(q)}`);
  const data = await res.json();
  renderList(data);
}

// ── Render password list ────────────────────────────
function renderList(data) {
  const list = document.getElementById('password-list');

  if (!data.length) {
    list.innerHTML = '<div class="empty">📭 No passwords found!</div>';
    return;
  }

  list.innerHTML = data.map(p => `
    <div class="password-item">
      <span class="id">🔑 ${p.id}</span>
      <span class="pass" id="pass-${p.id}">${p.password}</span>
      <div class="actions">
        <button class="btn-copy"  onclick="copyPassword('${p.id}', '${p.password}')">📋 Copy</button>
        <button class="btn-green" onclick="openModal('${p.id}', '${p.password}')">✏️</button>
        <button class="btn-red"   onclick="deletePassword('${p.id}')">🗑️</button>
      </div>
    </div>
  `).join('');
}

// ── Add password ────────────────────────────────────
async function addPassword() {
  const id   = document.getElementById('add-id').value.trim();
  const pass = document.getElementById('add-pass').value.trim();

  if (!id || !pass) return showToast('⚠️ ID and Password required!');

  const res = await fetch(API, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ id, password: pass }),
  });

  if (res.ok) {
    document.getElementById('add-id').value   = '';
    document.getElementById('add-pass').value = '';
    showToast('✅ Password saved!');
    loadAll();
  } else {
    showToast('❌ ID already exists!');
  }
}

// ── Delete password ─────────────────────────────────
async function deletePassword(id) {
  if (!confirm(`Delete password for "${id}"?`)) return;

  const res = await fetch(`${API}/${encodeURIComponent(id)}`, {
    method: 'DELETE'
  });

  if (res.ok) {
    showToast('🗑️ Deleted!');
    loadAll();
  } else {
    showToast('❌ Failed to delete!');
  }
}

// ── Copy password ───────────────────────────────────
function copyPassword(id, password) {
  navigator.clipboard.writeText(password).then(() => {
    showToast(`📋 Copied password for "${id}"!`);
  });
}

// ── Update modal ────────────────────────────────────
function openModal(id, password) {
  document.getElementById('update-old-id').value   = id;
  document.getElementById('update-new-id').value   = id;
  document.getElementById('update-new-pass').value = password;
  document.getElementById('update-modal').style.display = 'flex';
}

function closeModal() {
  document.getElementById('update-modal').style.display = 'none';
}

async function submitUpdate() {
  const oldId   = document.getElementById('update-old-id').value;
  const newId   = document.getElementById('update-new-id').value.trim();
  const newPass = document.getElementById('update-new-pass').value.trim();

  if (!newId || !newPass) return showToast('⚠️ Fields cannot be empty!');

  const res = await fetch(`${API}/${encodeURIComponent(oldId)}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ id: newId, password: newPass }),
  });

  if (res.ok) {
    closeModal();
    showToast('✅ Updated!');
    loadAll();
  } else {
    showToast('❌ Update failed!');
  }
}

// ── Toast notification ──────────────────────────────
function showToast(msg) {
  const t = document.getElementById('toast');
  t.textContent = msg;
  t.classList.add('show');
  setTimeout(() => t.classList.remove('show'), 2500);
}

// Load on start
loadAll();