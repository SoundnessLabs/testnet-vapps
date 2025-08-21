# Soundness Proof Notifier Bot

## Deskripsi
Bot sederhana untuk mengirim notifikasi ke Telegram/Discord ketika:
- Proof generation berhasil atau gagal
- Ada error pada blob id
- Node butuh tindakan cepat

## Manfaat
- Operator node tidak perlu mantau terminal terus
- Meningkatkan uptime operator
- Membantu dev mendeteksi bug lebih cepat

## Implementasi (High Level)
- Monitoring log/CLI output dari Soundness
- Jika ada event → trigger bot
- Kirim notifikasi via Telegram/Discord sesuai preferensi
