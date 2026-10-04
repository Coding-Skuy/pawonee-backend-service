> Versi: v1.0.0 | Status: disetujui | Menggantikan: -

# 40-TECHSTACK — Tumpukan Teknologi (pawonee-backend-service)

Mengacu: Pawonee-TownHall v1.0.0 (https://github.com/Coding-Skuy/Pawonee-TownHall).

## 1. Kondisi saat ini (tercatat)

- Rust 1.82.0, axum 0.8.1, tokio 1.42.0, sqlx 0.8.2 (postgres dan runtime-tokio), serde 1.0.215 dan serde_json 1.0.133, Postgres 16.4 (sumber: `Cargo.toml`, `README.md`).

## 2. Target standar emas (rencana)

- Rust stabil terbaru (edisi 2021), axum 0.8.4, tokio 1.x terbaru, sqlx 0.8.x terbaru (postgres dan runtime-tokio), Postgres 16.x terbaru. Pin eksak di `Cargo.toml`.
- Selaras divisi: KMP Kotlin 2.2.20 dan Compose 1.8.2 dan nav3 1.0.0; Rust axum 0.8.4; Python 3.12; web SvelteKit dan Bun terbaru; Postgres 16.x; database `pawonee`; JWT audiens `pawonee`.

## 3. Langkah penyesuaian (rencana — bukan eksekusi sekarang)

1. Kunci versi eksak pada `Cargo.toml` saat fase coding dimulai.
2. Selaraskan matriks `VERSIONS.md` di `pawonee-infra-devops` setelah fase coding dimulai.
3. Jalankan pemeriksaan build dan migrasi kering sebelum menaikkan versi.
4. Catat perubahan versi pada dokumen ini dan TownHall Pawonee-TownHall v1.0.0.

## Batasan

- Dokumen versi ini tidak mengubah manifes, kode, atau versi terpasang; semua target adalah rencana.
- Tidak ada migrasi framework atau kenaikan versi dalam dokumen ini.
- Model tetap mengikuti `:shared:pantry-resep` milik Pawonee; tidak ada duplikasi model baru.
- Perencanaan BRD, PRD, FSD, dan roadmap repo ini mengacu Pawonee-TownHall v1.0.0 dan tidak diduplikasi di sini.
