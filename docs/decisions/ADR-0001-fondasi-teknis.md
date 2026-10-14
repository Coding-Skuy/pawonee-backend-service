# ADR-0001 — Fondasi teknis pawonee-backend-service

Status: Diterima | Tanggal: 2026-10-14

## Konteks

Template backend memakai Axum 0.8.1 tetapi rute masih bersintaks `:id` dan endpoint belum mengikuti FSD 30. Sumber kebenaran: Pawonee-TownHall `platform/versions/v1.0.0` (https://github.com/Coding-Skuy/Pawonee-TownHall/tree/main/platform/versions/v1.0.0), beku 2026-10-07; lintas divisi: [KONTEKS-LINTAS-DIVISI.md](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/KONTEKS-LINTAS-DIVISI.md). Tiket fondasi: PWN-BE-001 (issue #1).

## Keputusan

1. Backend Rust + Axum 0.8 (path `{id}`), Tokio, SQLx + `migrate!`, tower-http (Trace + Timeout), tracing, uuid v7, validator, jsonwebtoken + argon2.
2. DB logis `pawonee` pada Postgres 16.x; DB per service, tidak ada akses silang divisi; `AppState { db: PgPool }`.
3. JWT akses 15 menit + refresh 7 hari dengan reuse-detection, audiens `pawonee`; API key per perangkat; service key least-privilege rotasi 90 hari.
4. Satu `AppError: IntoResponse` memetakan 400/401/404/409/422/429/500 ke kode baku (`STOK_KOSONG`, `PROFIL_TIDAK_ADA`, `JARINGAN_GAGAL`, `RESEP_TIDAK_ADA`, `JIWA_DI_LUAR_RENTANG`, `VALIDASI_GAGAL`, `BATAS_LAJU`).
5. Kontrak publik didefinisikan di OpenAPI 3.1 (`docs/api/openapi.yaml`) dengan referensi Scalar; perubahan kontrak = bump versi + ADR.

## Konsekuensi

- Rute lama (`/v1/preferensi/:id`, `/v1/resep/rekomendasi?bahan=`) harus diselaraskan ke kontrak FSD 30 lewat tiket PWN-BE-002..004.
- Pedaree dan klien membaca kontrak yang sama dari OpenAPI; perubahan tanpa bump versi dianggap pelanggaran.
- Rotasi service key 90 hari menjadi tugas operasional di `pawonee-infra-devops`.
