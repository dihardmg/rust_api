# Gunakan Rust nightly terbaru untuk kompatibilitas penuh
FROM rust:1.89.0

# Set working directory di dalam kontainer
WORKDIR /usr/src/app

# Salin semua file proyek ke kontainer
# Langkah ini penting agar cargo dapat menemukan semua file manifest
COPY . .

# Instal MySQL client agar skrip entrypoint dapat memeriksa koneksi database
RUN apt-get update && \
    apt-get install -y default-mysql-client && \
    rm -rf /var/lib/apt/lists/*

# Bangun aplikasi Rust Anda dalam mode rilis
# Ini akan mengompilasi semua dependensi dan binary
RUN cargo build --release

# Buat direktori uploads untuk file uploads
RUN mkdir -p uploads/banners

# Buat skrip entrypoint.sh dapat dieksekusi
RUN chmod +x entrypoint.sh

# Tetapkan entrypoint. Docker akan menjalankan skrip ini ketika kontainer dimulai.
ENTRYPOINT ["./entrypoint.sh"]