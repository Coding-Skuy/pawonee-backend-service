> Versi: v1.0.0 | Status: disetujui | Menggantikan: -

# 60-BLAST-RADIUS — Batas Context (pawonee-backend-service)

Mengacu: Pawonee-TownHall `platform/versions/v1.0.0` (https://github.com/Coding-Skuy/Pawonee-TownHall/tree/main/platform/versions/v1.0.0), beku 2026-10-07. Baris Pawonee townhall-roadmap §2: point of concern "12 resep inti; serapan ≥1.500 g"; batas context "G2-V/U/S; porsi 1–8 jiwa; DB `pawonee`; JWT audiens `pawonee`". Rujukan lintas divisi: KONTEKS §8/§9.

## 1. Boleh disentuh repo ini

- Seluruh kode `src/`, `migrations/`, dan `docs/api/` repo ini.
- DB logis `pawonee` (tabel `resep`, `profil_keluarga`, `riwayat_masak`, `sinyal_pedaree`).
- Membaca stok Pedaree lewat API/proksi Pedaree (bukan DB-nya).

## 2. Dilarang

- Query langsung ke DB Pedaree, Lumbung, atau divisi lain.
- Mengubah bentuk `Rekomendasi`/`Resep` tanpa memperbarui `:shared:pantry-resep` dan `models/schema.json`.
- Menghitung ulang rumus skor berbeda dari `:shared:pantry-resep`.
- Menyentuh DB divisi lain (DB per service; Titeny hanya read-only ke DB sumber).
- Memakai komoditas non-pilot (kentang/wortel/tomat/kol) untuk klaim pasokan agregasi Lumbung; hanya cabai, bawang, kangkung (KONTEKS §4).
- Menambah resep ke-13, mengubah definisi G2, atau mengubah audiens JWT tanpa PR TownHall + ADR.

## 3. Kontrak yang diproduksi

- API v1 `https://api.pawonee.chefgenie.id/v1` (header `X-Kontrak-Versi: v1`) dan OpenAPI 3.1 `docs/api/openapi.yaml`.
- Penerbitan JWT audiens `pawonee` + refresh; verifikasi API key perangkat.
- Tabel `sinyal_pedaree` sebagai sumber topik `sinyal.pedaree.v1` (dibaca pipa dan Pedaree).

## 4. Kontrak yang dikonsumsi

- Bank resep dan `models/schema.json` dari `pawonee-ai-models` (disinkron lewat migrasi).
- Stok Pedaree inventori (baca) dan harga Lumbung G2 per 100 g (wortel 1.200, kentang 1.500, tomat 1.800, kangkung 1.000, cabai 4.000).
- Rahasia (`DATABASE_URL`, kunci JWT, service/device API keys) dari `pawonee-infra-devops`.

## 5. Dampak bila berubah

- Perubahan skema DB butuh migrasi baru dan migrasi balik; memengaruhi 6 repo divisi.
- Perubahan audiens JWT memutus autentikasi semua klien dan backend.
- Perubahan kontrak tanpa pemberitahuan merusak rekomendasi pendamping Pedaree; beri tahu satu siklus rilis sebelumnya.

## Batasan

- Dokumen ini hanya memetakan batas dan risiko; tidak mengubah kode atau konfigurasi.
- Keputusan lintas divisi dirujuk ke `KONTEKS-LINTAS-DIVISI.md` dan TownHall Pawonee v1.0.0.
