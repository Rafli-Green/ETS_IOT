# 🏫 TROPHEUS: Sistem IoT Monitoring Suhu dan Kelembaban Ruang Kelas Tower 3 ITS

**Proyek IoT berbasis ESP32-S3 dan Sensor DHT22 untuk pemantauan suhu dan kelembaban ruang kelas secara real-time menggunakan protokol MQTT dan platform ThingsBoard Cloud.**

---

## 📖 Deskripsi Proyek

TROPHEUS (Temperature and Humidity Real-Time Observation System) dikembangkan untuk memantau kondisi termal ruang kelas Tower 3 ITS secara real-time dan terintegrasi dengan ThingsBoard Cloud.  
Sistem ini membaca data dari **sensor DHT22** menggunakan **mikrokontroler ESP32-S3**, kemudian mengirimkan data tersebut melalui **protokol MQTT** untuk divisualisasikan pada dashboard ThingsBoard.

Proyek ini mendukung implementasi _smart campus_ dengan sistem monitoring yang efisien, aman, dan mudah dikembangkan.

---

## 👨‍💻 Anggota Kelompok

**Kelompok 03 – D4 Teknologi Rekayasa Instrumentasi, Fakultas Vokasi ITS**  
- Ahmad Rafli Al Adzani (2042231001)  
- Valencia Christina Setiowardhani (2042231055)  
- Ahmad Radhy, S.Si., M.Si. *(Supervisor)*  

---

## ⚙️ Teknologi yang Digunakan

- **ESP32-S3** — mikrokontroler utama dengan WiFi & BLE  
- **DHT22 Sensor** — pengukur suhu dan kelembaban  
- **Rust Programming Language** — pemrograman firmware  
- **MQTT Protocol** — komunikasi data ringan  
- **ThingsBoard Cloud** — platform visualisasi dan telemetry IoT  
- **GNUPlot** — analisis grafik hasil pengujian  

---

## 🧩 Arsitektur Sistem
[DHT22 Sensor] → [ESP32-S3] → [MQTT Broker / ThingsBoard Cloud]


ESP32-S3 membaca data sensor DHT22 melalui GPIO4, kemudian mengirim hasilnya ke ThingsBoard Cloud menggunakan MQTT.  
Sistem bersifat **open-loop**, artinya hanya melakukan monitoring tanpa kontrol otomatis.

---

## 🔧 Wiring Diagram

| Komponen | Pin ESP32-S3 | Keterangan |
|-----------|---------------|------------|
| DHT22 (-) | GND           | Ground |
| DHT22 (OUT) | GPIO4       | Data sensor |
| DHT22 (+) | 3.3V          | Tegangan kerja |

---

## 🚀 Fitur Utama

- Monitoring suhu dan kelembaban real-time  
- Integrasi ThingsBoard untuk visualisasi data  
- Komunikasi data efisien menggunakan MQTT  
- Logging data telemetri berbasis timestamp  
- Sistem berbasis Rust yang aman dan stabil  

---

## 🧠 Cara Kerja Sistem

1. **Inisialisasi**  
   ESP32-S3 mengaktifkan koneksi WiFi dan sinkronisasi waktu NTP.  
2. **Pembacaan Sensor**  
   Data suhu dan kelembapan diambil dari sensor DHT22.  
3. **Publikasi MQTT**  
   Data dikirim ke ThingsBoard melalui protokol MQTT.  
4. **Visualisasi Data**  
   Data suhu & kelembapan ditampilkan di dashboard ThingsBoard secara real-time.  
5. **Analisis GNUPlot**  
   Data hasil monitoring dapat diproses menggunakan GNUPlot untuk melihat tren suhu dan kelembapan.

---

## 📊 Hasil Pengujian

- Sistem bekerja stabil selama **12 hari nonstop**  
- Rata-rata _latency_ pengiriman data: **< 0.3 detik**  
- Error pembacaan sensor: **< 1.5%**  
- Performa WiFi koneksi kampus: stabil dengan auto-reconnect  
- Hasil data ditampilkan melalui ThingsBoard dan diuji dengan GNUPlot  

---

## 🧩 Diagram Open Loop

+------------+ +-----------+ +-------------------+
| DHT22 | -----> | ESP32-S3 | -----> | ThingsBoard Cloud |
+------------+ +-----------+ +-------------------+


---

## 🧪 Analisis dan Pengembangan

- Data real-time membantu evaluasi kenyamanan termal ruang kelas Tower 3 ITS  
- Sistem dapat dikembangkan menjadi **closed-loop** untuk kontrol otomatis (AC/Fan)  
- Dapat ditambahkan notifikasi ketika suhu/kelembapan melewati ambang batas  
- Mendukung integrasi dengan **AI** untuk prediksi kenyamanan termal  

---

## 🏁 Kesimpulan

TROPHEUS berhasil dirancang dan diimplementasikan menggunakan **ESP32-S3**, **DHT22**, dan **ThingsBoard Cloud** untuk monitoring suhu & kelembapan ruang kelas secara efisien dan real-time.  
Sistem ini menjadi langkah awal menuju **Smart Campus ITS** yang mendukung kenyamanan belajar berbasis data.

---

## 📦 Lisensi

Proyek ini dikembangkan untuk keperluan akademik pada mata kuliah **Teknologi Internet of Things** di **Institut Teknologi Sepuluh Nopember (ITS)**.

© 2025 Ahmad Rafli Al Adzani & Valencia Christina Setiowardhani
