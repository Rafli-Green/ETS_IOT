# 🏫 TROPHEUS: Sistem IoT Monitoring Suhu dan Kelembapan Ruang Kelas Tower 3 ITS

**Proyek IoT berbasis ESP32-S3 dan sensor DHT22 untuk pemantauan suhu dan kelembapan ruang kelas Tower 3 ITS secara real-time menggunakan protokol MQTT dan platform ThingsBoard Cloud.**

---

## 📖 Deskripsi Umum

TROPHEUS (Temperature and Humidity Real-Time Observation System) dikembangkan untuk memantau kondisi termal ruang kelas Tower 3 ITS secara real-time dan terintegrasi dengan ThingsBoard Cloud.  
Sistem ini membaca data dari **sensor DHT22** menggunakan **mikrokontroler ESP32-S3**, kemudian mengirimkan data tersebut melalui **protokol MQTT** untuk divisualisasikan pada dashboard **ThingsBoard Cloud**.

> Proyek ini bertujuan mendukung inisiatif *Smart Campus ITS* melalui sistem monitoring yang efisien, aman, dan berbasis teknologi IoT modern.

---

## 👨‍💻 Anggota Kelompok

**Kelompok 03 – D4 Teknologi Rekayasa Instrumentasi, Fakultas Vokasi ITS**  
- Ahmad Rafli Al Adzani (2042231001)  
- Valencia Christina Setiowardhani (2042231055)  
- Ahmad Radhy, S.Si., M.Si. *(Supervisor)*  

---

## 🧠 Latar Belakang

Tower 3 ITS merupakan fasilitas utama Fakultas Vokasi dengan fungsi sebagai ruang belajar dan laboratorium mahasiswa.  
Kenyamanan termal ruangan berpengaruh langsung terhadap produktivitas dan fokus mahasiswa. Berdasarkan standar **ASHRAE 55-2023**, suhu ideal ruang kelas adalah **20°C–24°C** dan kelembapan **40%–60%**.  

TROPHEUS hadir sebagai solusi monitoring kondisi lingkungan ruang kelas menggunakan sensor dan cloud platform, sehingga data suhu dan kelembapan dapat dipantau secara real-time dan menjadi dasar pengelolaan kenyamanan ruang belajar.

<img width="580" height="325" alt="image" src="https://github.com/user-attachments/assets/12e0f37b-a5f0-4d0a-a93a-7cda70fcec7c" />


---

## 🎯 Tujuan dan Manfaat

### Tujuan
- Membangun sistem monitoring suhu dan kelembapan ruang kelas berbasis ESP32-S3 dan DHT22.  
- Mengirimkan data sensor ke ThingsBoard Cloud melalui protokol MQTT untuk visualisasi real-time.

### Manfaat
- Menjadi media pembelajaran IoT bagi mahasiswa.
- Menyediakan data real-time bagi fakultas untuk analisis kenyamanan ruangan.
- Mendorong konsep **Smart Campus** ITS.

---

## ⚙️ Komponen Sistem

| Komponen | Fungsi | Spesifikasi |
|-----------|---------|--------------|
| **ESP32-S3** | Mikrokontroler utama | Dual-core Xtensa LX7 @ 240 MHz, WiFi + BLE |
| **DHT22 Sensor** | Sensor suhu & kelembapan | Akurasi ±0.5°C, ±2–5% RH |
| **MQTT** | Protokol komunikasi | Publish/Subscribe |
| **ThingsBoard Cloud** | Platform visualisasi | Dashboard & data telemetry |
| **GNUPlot** | Analisis data | Grafik tren suhu dan kelembapan |

---

## 🧩 Arsitektur Sistem

<img width="541" height="159" alt="image" src="https://github.com/user-attachments/assets/ccaa62d4-817c-4ea9-b962-e501df46e21b" />


ESP32-S3 membaca data dari sensor DHT22 melalui GPIO, memvalidasi hasilnya, dan mengirimkan data ke ThingsBoard Cloud menggunakan MQTT.  
Sistem bersifat **open-loop**, hanya melakukan pemantauan tanpa pengendalian otomatis.

---

## 🔄 Flowchart Sistem

<img width="268" height="772" alt="image" src="https://github.com/user-attachments/assets/879c65ba-e13f-4e7b-bc01-e4e5e73c307c" />


Alur sistem TROPHEUS:
1. **Inisialisasi** – Sistem mengaktifkan WiFi dan mempersiapkan koneksi MQTT.  
2. **Pembacaan Sensor** – DHT22 membaca suhu & kelembapan.  
3. **Pemrosesan Data** – Validasi & formatting MQTT.  
4. **Publikasi Data** – Pengiriman data ke ThingsBoard.  
5. **Monitoring Dashboard** – Visualisasi real-time.

---

## ⚡ Wiring Diagram

<img width="531" height="113" alt="image" src="https://github.com/user-attachments/assets/c14a1a62-7ff5-4d82-bee9-f51912feeed1" />


| Komponen | Pin ESP32-S3 | Keterangan |
|-----------|---------------|------------|
| DHT22 (-) | GND | Ground |
| DHT22 (OUT) | GPIO4 | Jalur data sensor |
| DHT22 (+) | 3.3V | Sumber daya |

---

## 💻 Pengujian Sistem

Flashing firmware Rust ke ESP32-S3:
```bash
espflash flash --partition-table partitions.csv target/xtensa-esp32s3-espidf/debug/dev --monitor --port /dev/ttyACM0
