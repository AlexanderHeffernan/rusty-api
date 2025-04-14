# RustAPIStarter
A secure, lightweight Rust API template for rapid backend development. Features **HTTPS**, **user authentication**, **privilege levels**, and **rate limiting**.

## Features
- **HTTPS**: Built-in TLS for secure communication.
- **Authentication**: JWT-based user login with refresh tokens.
- **Privilege Levels**: Role-based access control (admin, user, etc.).
- **Rate Limiting**: Configurable limits to prevent abuse.
- **Modular**: Simple to extend with custom routes, user privilege levels and user data fields.

## Getting Started
### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [OpenSSL](https://www.openssl.org/) (for HTTPS certificates)
- [sqlite3](https://sqlite.org/download.html) (for user authentication)

### Installation
#### 1. Clone the repository:
```bash
git clone https://github.io/AlexanderHeffernan/RustAPIStarter.git
cd RustAPIStarter
```
#### 2. Generate self-signed certificates
```bash
mkdir -p certs
openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem
```
#### 3. Prepare the SQLite database
```bash
sqlite3 user.db
```
```sql
CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  email TEXT NOT NULL UNIQUE,
  api_key TEXT NOT NULL UNIQUE,
  privilege_level INTEGER NOT NULL
);
.exit
```
This will create a user.db file with a users table containing the required fields: id, email, api_key, and privilege_level.
email, api_key, and privilege_level.
#### 4. Build and run the project
```bash
cargo run
```
The API will start on `https://localhost:8443` by default.

## Usage
- Add routes in `src/routes/`.
- Define models in `src/models/`.
- Check `src/routes/` for sample APIs (e.g. `admin_demo.rs` and `guest_demo.rs`).
- Test endpoints with curl (e.g., `https://localhost:8443/guest_demo`.

## Contributing
Issues and pull requests are welcome!

## License
- This code is provided as-is, without warranty of any kind.
- You are free to use, modify, and distribute this code as part of your projects.
