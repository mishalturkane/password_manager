# 🔐 Password Manager

A password manager built in **Rust** with two versions — a CLI app and a Web app.

---

## 📦 Versions

### V1 — CLI App (`main` branch)
A terminal-based password manager using Rust + PostgreSQL.

### V2 — Web App (`v2-web-app` branch)
A full web app with Axum REST API + Doodle-style HTML/CSS frontend.

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust 🦀 |
| Backend (V2) | Axum |
| Database | PostgreSQL |
| DB Library | SQLx |
| Encryption | AES-256 (magic-crypt) |
| Frontend (V2) | Plain HTML + Tailwind CSS 4 + Vanilla JS |
| Fonts | Poppins + Space Grotesk |

---

## ✨ Features

### V1 CLI
- ➕ Add password
- 🔍 Get password by ID
- 📋 List all IDs
- ✏️ Update ID and/or password
- 🗑️ Delete password
- ⚡ Auto creates DB and table on first run

### V2 Web App
- 🔐 AES-256 encrypted passwords in database
- 🔍 Live search by ID
- 📋 One-click copy password
- ✏️ Update password via modal
- 🗑️ Delete password
- 🎨 Doodle-style UI
- ⚡ Auto creates DB and table on first run

---

## 🚀 Getting Started

### Prerequisites
- Rust installed → [rustup.rs](https://rustup.rs)
- PostgreSQL installed and running

### 1. Clone the repo

```bash
git clone https://github.com/mishalturkane/password_manager.git
cd password_manager
```

### 2. Create `.env` file

```env
DATABASE_URL=postgres://postgres:yourpassword@localhost:5432/password_manager_db
ENCRYPTION_KEY=your_secret_key_here        # V2 only
```

### 3. Run V1 (CLI)

```bash
git checkout main
cargo run
```

### 4. Run V2 (Web App)

```bash
git checkout v2-web-app
cargo run
```

Then open → `http://localhost:8080`

> ✅ Database and table are created automatically on first run — no manual SQL needed!

---

## 📁 Project Structure

### V1 — CLI
```
src/
├── main.rs
├── db.rs
├── crypto.rs
├── models/
│   └── password.rs
├── repository/
│   └── password_repo.rs
└── cli/
    ├── mod.rs
    ├── handlers.rs
    └── input.rs
```

### V2 — Web App
```
src/
├── main.rs
├── db.rs
├── crypto.rs
├── models/
│   └── password.rs
├── repository/
│   └── password_repo.rs
├── handlers/
│   └── password_handler.rs
└── routes/
    └── mod.rs
static/
├── index.html
├── style.css
└── script.js
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

Passwords are stored **AES-256 encrypted** in the database. Plain text is never stored. Set a strong `ENCRYPTION_KEY` in your `.env` file.

> ⚠️ This project is for learning purposes. For production use, consider adding authentication.

---

## 👨‍💻 Author

**Mishal Turkane**
- GitHub → [@mishalturkane](https://github.com/mishalturkane)

---

## 📄 License

MIT License — free to use and modify!