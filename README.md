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
