# 🛡️ Auth Platform – Rust gRPC + Next.js (SSR)

A **production-ready authentication & authorization platform** built with **Rust**, **gRPC**, **PostgreSQL**, and **Next.js (SSR)**.  
Designed for **high performance**, **type safety**, and **containerized deployment**.

---

## ✨ Features

- 🔐 Authentication & Authorization service (Register / Login / Validate Token)
- 🚀 gRPC-only backend (no REST)
- 🦀 Rust + Tonic (high-performance async server)
- 🗄 PostgreSQL with SQLx
- 🔑 JWT-based auth (HttpOnly cookies)
- 🌐 Next.js (TypeScript) client with **SSR**
- 🐳 Fully containerized with Docker & Docker Compose
- 🤖 CI/CD pipeline (Docker image build & publish)
- 📦 Production-ready project structure

---

## 🏗️ Architecture Overview
```bash

┌────────────┐        gRPC      ┌──────────────┐ 
│ Next.js    │ ───────────────▶ │ Auth Service │
│ (SSR)      │                  │  Rust + Tonic│
└────────────┘                  └───────┬──────┘
                                        │
                                        ▼
                                ┌────────────┐
                                │ PostgreSQL │
                                └────────────┘
```
- Frontend communicates with backend via **gRPC**
- SSR authentication using **secure HttpOnly cookies**
- Auth service is **stateless** (JWT)

---

## 📂 Project Structure
```
htdung-core/
├── auth-service/
│   ├── proto/                  # gRPC proto definitions
│   ├── src/
│   │   ├── auth/               # JWT & password logic
│   │   ├── db/                 # SQLx repositories
│   │   ├── grpc/               # gRPC service implementation
│   │   ├── error.rs            # Domain errors → gRPC Status
│   │   ├── state.rs            # Shared application state
│   │   └── main.rs
│   ├── build.rs                # gRPC code generation
│   ├── Cargo.toml
│   └── Dockerfile
│
├── web-client/
│   ├── pages/
│   │   ├── api/auth/           # gRPC → HTTP bridge
│   │   └── dashboard.tsx
│   ├── lib/                    # gRPC client & auth helpers
│   ├── Dockerfile
│   └── package.json
│
├── docker-compose.yml
├── .github/workflows/           # CI/CD pipelines
├── .gitignore
└── README.md
```
---

## 🧰 Tech Stack

### Backend
- Rust
- Tonic (gRPC)
- Tokio
- SQLx (PostgreSQL)
- JSON Web Tokens (JWT)
- Argon2 password hashing

### Frontend
- Next.js
- TypeScript
- Server-Side Rendering (SSR)
- gRPC client (`@grpc/grpc-js`)

### Infrastructure
- Docker & Docker Compose
- GitHub Actions (CI/CD)
- Docker Hub registry

---

## 🚀 Getting Started

### Prerequisites
- Docker
- Docker Compose
- Docker Hub account (for pulling images)

---

### Pull & Run Services

```bash
docker login
docker compose pull
docker compose up -d
```
### Verify Services
```bash
docker compose ps
docker compose logs auth-service
docker compose logs web-client
```
	•	Web client: http://localhost:3000
	•	gRPC auth service: localhost:50051

⸻

### 🔐 Authentication Flow
	1.	User registers or logs in via Next.js
	2.	Next.js API routes call the gRPC auth-service
	3.	Auth-service generates JWT
	4.	JWT stored in HttpOnly cookie
	5.	SSR validates token on every request
	6.	User info available server-side

⸻

### 🧪 gRPC API
```code
service AuthService {
  rpc Register (RegisterRequest) returns (AuthResponse);
  rpc Login (LoginRequest) returns (AuthResponse);
  rpc ValidateToken (ValidateTokenRequest) returns (ValidateTokenResponse);
}
```


⸻

### 🐳 Docker & CI/CD
	•	Multi-stage Docker builds
	•	protoc installed in builder stage
	•	CI pipeline:
	•	Builds Docker images
	•	Pushes to Docker Hub
	•	Tags with latest and commit SHA

### Update deployment:
```
docker compose pull
docker compose up -d
```

⸻

### 🔒 Security
	•	Passwords hashed with Argon2
	•	JWT stored as HttpOnly cookies
	•	No token access from client JavaScript
	•	gRPC-only backend (reduced attack surface)
	•	Ready for RBAC and mTLS

⸻

### 📌 Roadmap
	•	Refresh tokens
	•	Role-based access control (RBAC)
	•	gRPC health checks & reflection
	•	Observability (Tracing, Prometheus)
	•	Kubernetes manifests
	•	mTLS between services

⸻

🏁 Status
```
✅ Fully working
✅ CI/CD enabled
✅ Docker-ready
✅ Production-grade SSR authentication