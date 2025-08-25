# Attendance API (Actix + SeaORM)

Quick start:
1. Copy `.env.example` to `.env` and set DATABASE_URL.
2. Start MySQL and create DB (or use docker-compose).
3. Run migrations: `cargo migrate up`
4. Run server: `cargo run --release`

Endpoints:
- POST /api/attendance/clockin  -> { "user_id": "EMP001" }
- POST /api/attendance/clockout -> { "user_id": "EMP001" }
- GET  /api/attendance?user_id=EMP001&limit=10
