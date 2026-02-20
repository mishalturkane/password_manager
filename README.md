# 🔐 Password Manager

A password manager built in **Rust** — from a CLI app to a full Web App with REST API.

> Built as a learning project to explore Rust, Axum, PostgreSQL, and AES encryption.

## 🌐 Live Demo

**[https://password-manager.mishalturkane.xyz/](https://password-manager.mishalturkane.xyz/)**

> Hosted on Render · PostgreSQL on Render · Custom domain via GoDaddy

---

## 📦 Versions

| Version | Type | Status |
|---|---|---|
| **V1** | CLI Terminal App | ✅ Done |
| **V2** | Web App (Axum + HTML) | ✅ Live |

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust 🦀 |
| Backend | Axum |
| Database | PostgreSQL |
| DB Library | SQLx |
| Encryption | AES-256 (magic-crypt) |
| Frontend | Plain HTML + Tailwind CSS 4 + Vanilla JS |
| Fonts | Poppins + Space Grotesk |
| Runtime | Tokio (async) |
| Hosting | Render |
| Domain | GoDaddy → mishalturkane.xyz |

---

## ✨ Features

### V1 — CLI
- ➕ Add password
- 🔍 Get password by ID
- 📋 List all passwords
- ✏️ Update ID and/or password
- 🗑️ Delete password
- ⚡ Auto creates DB and table on first run

### V2 — Web App
- 🔐 AES-256 encrypted passwords in database
- 🌐 REST API with Axum
- 🔍 Live search by ID
- 📋 One-click copy password to clipboard
- 👁️ Show/hide password toggle
- ✏️ Update password via modal popup
- 🗑️ Delete password
- 🎨 Doodle-style UI (Tailwind CSS 4)
- ⚡ Auto creates DB and table on first run
- 🚀 Deployed on Render with custom domain

---

## 🚀 Getting Started

### Prerequisites
- Rust → [rustup.rs](https://rustup.rs)
- PostgreSQL installed and running

### 1. Clone the repo

```bash
git clone https://github.com/mishalturkane/password_manager.git
cd password_manager
```

### 2. Create `.env` file

```env
DATABASE_URL=postgres://postgres:yourpassword@localhost:5432/password_manager_db
ENCRYPTION_KEY=your_strong_secret_key_here
PORT=8080
```

### 3. Run V1 (CLI)

```bash
cargo run --bin password_manager
```

### 4. Run V2 (Web App)

```bash
cargo run
```

Then open browser → `http://localhost:8080`

> ✅ No manual SQL needed — DB and table are created automatically on first run!

---

## 📁 Project Structure

```
password_manager/
├── Cargo.toml
├── Dockerfile
├── .env
├── static/                        # V2 Frontend
│   ├── index.html
│   ├── style.css
│   └── script.js
└── src/
    ├── main.rs                    # Entry point
    ├── db.rs                      # Auto DB + table setup
    ├── crypto.rs                  # AES-256 encrypt/decrypt
    ├── models/
    │   └── password.rs            # Structs
    ├── repository/
    │   └── password_repo.rs       # All DB queries
    ├── handlers/
    │   └── password_handler.rs    # Axum route handlers
    ├── routes/
    │   └── mod.rs                 # Route definitions
    └── cli/                       # V1 CLI
        ├── mod.rs
        ├── handlers.rs
        └── input.rs
```

---

## 🔗 API Endpoints (V2)

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/passwords` | Get all passwords |
| `GET` | `/api/passwords/:id` | Get by ID |
| `GET` | `/api/passwords/search?q=` | Search by ID |
| `POST` | `/api/passwords` | Add new password |
| `PUT` | `/api/passwords/:id` | Update password |
| `DELETE` | `/api/passwords/:id` | Delete password |

---

## 🔐 Security Note

Passwords are stored **AES-256 encrypted** in the database — plain text is never stored.

> ⚠️ This is a learning project. For production, consider adding authentication (JWT) and HTTPS.

---

## 🗺️ Roadmap

- [x] V1 — CLI CRUD with PostgreSQL
- [x] V2 — Web App with Axum REST API + Doodle UI
- [x] Deploy on Render with custom domain
- [ ] V3 — Auth (Login/Register with JWT)

---

## 👨‍💻 Author

**Mishal Turkane**
- 🌐 Portfolio → [mishalturkane.xyz](https://www.mishalturkane.xyz/)
- 🐙 GitHub → [@mishalturkane](https://github.com/mishalturkane)

---

## 📄 License

MIT License — free to use and modify!