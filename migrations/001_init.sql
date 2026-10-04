-- Migrasi awal database `pawonee`.
-- Tabel resep/preferensi selaras dengan modul `:shared:pantry-resep` milik Pawonee.

CREATE TABLE IF NOT EXISTS resep (
    id TEXT PRIMARY KEY,
    nama TEXT NOT NULL,
    bahan JSONB NOT NULL,
    langkah JSONB NOT NULL,
    menit INTEGER NOT NULL CHECK (menit > 0),
    level_pedas SMALLINT NOT NULL CHECK (level_pedas BETWEEN 0 AND 5),
    estimasi_biaya BIGINT NOT NULL CHECK (estimasi_biaya >= 0),
    grade SMALLINT NOT NULL DEFAULT 2
);

CREATE TABLE IF NOT EXISTS preferensi (
    pengguna_id TEXT PRIMARY KEY,
    level_pedas_maks SMALLINT NOT NULL DEFAULT 3 CHECK (level_pedas_maks BETWEEN 0 AND 5),
    alergi JSONB NOT NULL DEFAULT '[]'::jsonb,
    anggaran_maks BIGINT NOT NULL DEFAULT 50000 CHECK (anggaran_maks >= 0)
);

INSERT INTO resep (id, nama, bahan, langkah, menit, level_pedas, estimasi_biaya, grade) VALUES
('tempe-tumis-bawang', 'Tempe Tumis Bawang',
 '[{"nama":"tempe","jumlah":"1 papan"},{"nama":"bawang","jumlah":"5 siung"}]',
 '["Iris tempe dan bawang.","Tumis bawang hingga harum, masukkan tempe.","Bumbui dan masak 8 menit."]',
 15, 1, 12000, 2),
('sayur-bening-bayam', 'Sayur Bening Bayam',
 '[{"nama":"bayam","jumlah":"1 ikat"},{"nama":"bawang","jumlah":"3 siung"}]',
 '["Didihkan 600 ml air.","Masukkan bawang lalu bayam.","Bumbui dan masak 5 menit."]',
 10, 0, 8000, 2)
ON CONFLICT (id) DO NOTHING;
