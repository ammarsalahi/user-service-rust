# User Service API

A high-performance user management service built with **Rust**, **Axum**, and **SurrealDB**. Features JWT authentication, user management, and validation workflows with full OpenAPI/Swagger documentation.

## Features

- **Authentication**: Login, registration, password reset flows with JWT tokens (access + refresh)
- **User Management**: CRUD operations for users with role-based fields
- **Validation System**: One-time verification codes for email confirmation and password reset
- **API Documentation**: Interactive Swagger UI at `/swagger`
- **Database**: SurrealDB for flexible document-graph storage
- **Security**: Argon2 password hashing, secure JWT tokens

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 2024 Edition |
| Web Framework | Axum 0.8 |
| Database | SurrealDB 2.x (WebSocket protocol) |
| Auth | JSON Web Tokens (HS256) |
| Password Hashing | Argon2 |
| API Docs | utoipa + Swagger UI |
| Async Runtime | Tokio |

## Quick Start

### Prerequisites

- Rust 1.80+
- SurrealDB 2.x running on `ws://127.0.0.1:8000`

### Installation

```bash
# Clone and enter the project
git clone <repository-url>
cd user-service-rust

# Set required environment variable
export JWT_SECRET="your-super-secret-key-at-least-32-chars"

# Run the server
cargo run
```

Server starts at `http://localhost:3000`

### SurrealDB Setup

```bash
# Start SurrealDB (in a separate terminal)
surreal start --user root --pass root --bind 0.0.0.0:8000
```

## API Documentation

Interactive Swagger UI available at: **http://localhost:3000/swagger-ui**

OpenAPI JSON spec at: **http://localhost:3000/api-docs/openapi.json**

## API Endpoints

### Authentication (`/auth`)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/auth/login` | User login, returns access + refresh tokens |
| `POST` | `/auth/register` | Register new user, returns tokens |
| `POST` | `/auth/forget-password` | Request password reset (returns 202) |
| `POST` | `/auth/change-password` | Change password with old/new |
| `POST` | `/auth/validation/check` | Verify a validation code |

### Users (`/users`)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/users` | Create a new user |
| `GET` | `/users` | List all users |
| `GET` | `/users/{id}` | Get user by ID |
| `PUT` | `/users/{id}` | Update user by ID |
| `DELETE` | `/users/{id}` | Delete user by ID |

### Validations (`/validations`)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/validations` | Create validation record |
| `GET` | `/validations` | List all validations |
| `GET` | `/validations/{id}` | Get validation by ID |
| `PUT` | `/validations/{id}` | Update validation by ID |
| `DELETE` | `/validations/{id}` | Delete validation by ID |

## Request/Response Examples

### Login

```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password123"}'
```

**Response (200):**
```json
{
  "access": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### Register

```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com", "password": "securepass123"}'
```

### Create User

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jane Smith",
    "username": "janesmith",
    "password": "hashed_password_here",
    "sex": "Female",
    "email": "jane@example.com",
    "is_admin": false,
    "is_confirmed": true
  }'
```

### List Users

```bash
curl http://localhost:3000/users
```

## Data Models

### User
```json
{
  "name": "string (optional)",
  "username": "string",
  "password": "string (hashed, not returned in responses)",
  "sex": "Male | Female | Other",
  "email": "string",
  "is_admin": "boolean (optional)",
  "is_confirmed": "boolean (optional)"
}
```

### AuthResponse
```json
{
  "access": "string (JWT, 1 hour expiry)",
  "refresh": "string (JWT, 24 hour expiry)"
}
```

### Validation
```json
{
  "email": "string (optional)",
  "user_id": "string (optional)",
  "action": "Register | ForgetPassword",
  "is_expired": "boolean (optional)",
  "code": "string (6-digit code)"
}
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `JWT_SECRET` | Yes | Secret key for signing JWT tokens (min 32 chars) |

## Project Structure

```
src/
├── main.rs              # Application entry point, OpenAPI config
├── common.rs            # Shared types (DB state)
├── enums.rs             # Sex, ActionType enums
├── error.rs             # Error types & HTTP response mapping
├── models/              # Request/Response DTOs
│   ├── auth.rs          # Login, Register, AuthResponse, etc.
│   ├── user.rs          # User, UpdateUser, UserResponse
│   └── validation.rs    # Validation, CheckValidation
├── repositories/        # Data access layer
│   ├── auth.rs          # Auth repository
│   ├── user.rs          # User repository
│   └── validation.rs    # Validation repository
├── routes/              # HTTP route handlers
│   ├── auth.rs          # Auth endpoints
│   ├── user.rs          # User endpoints
│   └── validation.rs    # Validation endpoints
└── traits.rs            # Database trait definitions
```

## Development

### Build

```bash
cargo build
```

### Run with auto-reload (requires `cargo-watch`)

```bash
cargo watch -x run
```

### Check formatting

```bash
cargo fmt --check
```

### Lint

```bash
cargo clippy
```

## Production Deployment

1. Set strong `JWT_SECRET` (64+ random chars)
2. Use HTTPS/TLS termination (nginx, Traefik, etc.)
3. Configure SurrealDB with authentication and persistence
4. Run behind a process manager (systemd, Docker, etc.)

### Docker Example

```dockerfile
FROM rust:1.80 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/user-service-rust /usr/local/bin/
EXPOSE 3000
CMD ["user-service-rust"]
```

## License

MIT License - see [LICENSE](LICENSE) for details.
