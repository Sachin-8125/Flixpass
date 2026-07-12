# Flixpass 🎬

Flixpass is a modern, full-stack movie ticket booking application. It features a high-performance **Rust** backend built with **Axum** and **Diesel ORM**, and a sleek, interactive **React** frontend powered by **Vite** and **Tailwind CSS**.

---

## 🏗️ Architecture & Tech Stack

Flixpass is structured as a monorepo containing two main components:

### 1. Backend (`/backend`)
*   **Language:** Rust (2024 edition)
*   **Web Framework:** [Axum](https://github.com/tokio-rs/axum) (v0.8) for fast, type-safe, and asynchronous routing.
*   **Database & ORM:** [Diesel](https://diesel.rs/) (v2.3) with PostgreSQL.
*   **Authentication:** JSON Web Tokens ([jsonwebtoken](https://github.com/Keats/jsonwebtoken)) and secure password hashing using [Argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2).
*   **Runtime:** [Tokio](https://tokio.rs/) for asynchronous I/O.
*   **CORS & Logging:** [Tower HTTP](https://github.com/tower-rs/tower-http) and [Tracing](https://github.com/tokio-rs/tracing) for request tracking and observability.

### 2. Frontend (`/frontend`)
*   **Framework:** [React 19](https://react.dev/)
*   **Build Tool:** [Vite 8](https://vite.dev/)
*   **Styling:** [Tailwind CSS v4](https://tailwindcss.com/) with modern CSS-first configuration.
*   **Icons:** [Lucide React](https://lucide.dev/) for crisp vector icons.

---

## 📊 Database Schema

The PostgreSQL database (managed via Diesel ORM) consists of the following core tables:

```mermaid
erDiagram
    USERS {
        text id PK
        text name
        text email
        text password_hash
        text role "USER | ADMIN"
        timestamptz created_at
        timestamptz updated_at
    }
    MOVIES {
        text id PK
        text title
        text genre
        text rating
        int duration_minutes
        text synopsis
        text poster_tone
        bool is_active
        timestamptz created_at
        timestamptz updated_at
    }
    SHOWTIMES {
        text id PK
        text movie_id FK
        timestamptz starts_at
        text screen
        int price_cents
        int total_seats
        timestamptz created_at
        timestamptz updated_at
    }
    BOOKINGS {
        text id PK
        text user_id FK
        text showtime_id FK
        int_array seats
        int total_cents
        text status "CONFIRMED | CANCELLED"
        timestamptz created_at
        timestamptz updated_at
    }

    USERS ||--o{ BOOKINGS : places
    MOVIES ||--o{ SHOWTIMES : schedules
    SHOWTIMES ||--o{ BOOKINGS : has
```

---

## 🚀 Getting Started

### Prerequisites
*   [Rust & Cargo](https://www.rust-lang.org/tools/install) (v1.80+ recommended)
*   [Node.js](https://nodejs.org/) (v18+ recommended) & npm
*   [PostgreSQL](https://www.postgresql.org/) database server
*   [Diesel CLI](https://diesel.rs/guides/getting-started) (optional, for running migrations manually)

---

### 1. Backend Setup

1.  Navigate to the backend directory:
    ```bash
    cd backend
    ```

2.  Create your environment configuration file:
    ```bash
    cp .env.example .env
    ```
    Open `.env` and configure your database connection string and JWT secret:
    ```env
    DATABASE_URL="postgresql://postgres:postgres@localhost:5432/movie_booking?schema=public"
    JWT_SECRET="your-super-secure-long-jwt-secret-key"
    FRONTEND_ORIGIN="http://localhost:5173"
    RUST_LOG="backend=debug,tower_http=info"
    ```

3.  Set up the database and run migrations:
    ```bash
    # Install diesel_cli with postgres feature if you haven't already
    cargo install diesel_cli --no-default-features --features postgres

    # Setup database and run migrations
    diesel database setup
    ```

4.  Start the Axum development server:
    ```bash
    cargo run
    ```
    The server will start listening on `http://localhost:8080`.

---

### 2. Frontend Setup

1.  Navigate to the frontend directory:
    ```bash
    cd frontend
    ```

2.  Create your environment configuration file:
    ```bash
    cp .env.example .env
    ```
    Specify the API URL where the Rust backend is running:
    ```env
    VITE_API_URL=http://localhost:8080
    ```

3.  Install dependencies:
    ```bash
    npm install
    ```

4.  Start the Vite development server:
    ```bash
    npm run dev
    ```
    The application will be accessible at `http://localhost:5173` (or the port specified in your terminal).

---

## 🔌 API Endpoints

### Public Endpoints
*   `GET /health` - Health check status.
*   `POST /api/auth/register` - Register a new user.
*   `POST /api/auth/login` - Authenticate user and receive JWT.
*   `GET /api/movies` - List all active movies and their respective showtimes.

### Protected Endpoints (Requires `Authorization: Bearer <token>`)
*   `GET /api/bookings/me` - Retrieve current user's bookings.
*   `POST /api/bookings` - Book seats for a specific showtime.

### Admin Endpoints (Requires Admin Role JWT)
*   `POST /api/admin/movies` - Create a new movie.
*   `POST /api/admin/showtimes` - Create a new showtime.

---

## ✨ Features

*   **Interactive Seat Selection:** Dynamic map displaying available and booked seats in real-time.
*   **Secure Authentication:** Password hashing via Argon2 and stateless session management with JWT.
*   **Comprehensive Booking Flow:** Users can select a movie, choose a showtime, pick seats, and confirm their ticket bookings.
*   **Responsive Design:** Beautiful and adaptive user interface styled with Tailwind CSS, supporting mobile and desktop layouts.
*   **Robust Error Handling:** Custom JSON error payloads returned by the Rust API for seamless frontend debugging and user feedback.
