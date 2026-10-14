> Versi: v1.0.0 | Status: disetujui | Menggantikan: -

# 40-TECHSTACK — Tumpukan Teknologi (pawonee-backend-service)

Mengacu: Pawonee-TownHall `platform/versions/v1.0.0` (https://github.com/Coding-Skuy/Pawonee-TownHall/tree/main/platform/versions/v1.0.0), beku 2026-10-07. Tidak menduplikasi BRD/PRD/FRD/FSD/SRS; hanya mengunci stack repo ini.

## 1. Matriks stack

| Komponen | Kondisi saat ini (tercatat) | Target terkunci v1.0.0 |
|---|---|---|
| Rust | 1.82.0 | stabil terbaru, edisi 2021 (pin eksak di `Cargo.toml`) |
| Web framework | axum 0.8.1 (rute `:id`) | axum 0.8.x (path `{id}`) |
| Async | tokio 1.42.0 | tokio 1.x terbaru |
| DB | sqlx 0.8.2, Postgres 16.4 | sqlx 0.8.x + `migrate!`, Postgres 16.x |
| Middleware | tower-http 0.6.1 (cors) | tower-http (Trace + Timeout + CORS) |
| Observabilitas | belum ada | tracing |
| ID | belum ada | uuid v7 |
| Auth | belum ada | jsonwebtoken + argon2 |
| Uji | axum-test 16.0.0 | `#[sqlx::test]` + uji kontrak |

## 2. Drift yang tercatat

- Rute `/v1/preferensi/:id` memakai sintaks Axum 0.7; wajib `{id}` pada Axum 0.8.
- Endpoint saat ini (`GET /kesehatan`, `GET /v1/resep/rekomendasi?bahan=`, `GET|PUT /v1/preferensi/{id}`) belum sesuai FSD 30 (`GET /v1/kesehatan`, `/resep`, `/resep/{id}`, `/rekomendasi`, `/stok`, `/profil/{idKeluarga}`); diselaraskan lewat PWN-BE-004.
- Belum ada JWT/auth, `AppState`, `AppError`; masing-masing PWN-BE-002 dan PWN-BE-003.

## Batasan

- Dokumen ini tidak mengubah manifes atau kode; kondisi saat ini dicatat apa adanya dari manifes/README repo.
- Versi dikunci eksak di manifes saat kickoff coding; perubahan versi dicatat di dokumen ini lewat versi dokumen berikutnya dan ADR baru.
- JWT audiens `pawonee`, DB logis `pawonee`, dan `:shared:pantry-resep` milik Pawonee tidak berubah oleh dokumen ini.
