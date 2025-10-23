# 🌍 **TROPHEUS**  
### *Sistem Monitoring Suhu dan Kelembaban Udara Real-Time dan Terintegrasi untuk Ruang Kelas*  

---

## 🧭 **Deskripsi Umum**
Perkembangan teknologi **Internet of Things (IoT)** telah membawa dampak besar dalam berbagai sektor, termasuk bidang pendidikan dan pengelolaan fasilitas kampus.  
Pemanfaatan IoT memungkinkan integrasi antara sensor, mikrokontroler, dan platform cloud untuk memantau serta mengelola kondisi lingkungan secara otomatis dan real-time.  

Dalam konteks ruang kelas, kenyamanan termal memiliki peran penting terhadap fokus, kesehatan, dan produktivitas mahasiswa. Berdasarkan standar **ANSI/ASHRAE 55-2023**, suhu ideal berada di rentang **20°C–24°C**, sementara menurut **Permenkes RI No.1077/Menkes/Per/V/2011**, kelembapan ideal untuk aktivitas manusia berkisar antara **40%–60%**.  

Untuk memenuhi kebutuhan tersebut, dikembangkanlah **TROPHEUS** (*Temperature and Humidity Real-Time Observation and Processing Hub for Educational Utility Spaces*), yaitu sistem pemantauan suhu dan kelembaban udara berbasis **ESP32-S3** dan **sensor DHT22** yang terintegrasi dengan **ThingsBoard Cloud** menggunakan **protokol MQTT**.  

TROPHEUS dirancang sebagai langkah awal menuju konsep **Smart Campus ITS**, di mana setiap ruang kelas dapat dipantau secara digital untuk mendukung efisiensi energi, kenyamanan belajar, dan pengambilan keputusan berbasis data.  

---

## 👥 **Nama Kelompok 3**
- **Ahmad Rafli Al Adzani (2042231001)**  
- **Valencia Christina Setiowardhani (2042231055)**  
- **Ahmad Radhy, S.Si., M.Si.** *(Supervisor)*  

📍 *Program Studi D4 Teknologi Rekayasa Instrumentasi*  
📍 *Departemen Teknik Instrumentasi*  
📍 *Fakultas Vokasi – Institut Teknologi Sepuluh Nopember (ITS)*  
📆 *Tahun 2025*  

---

## ⚙️ **Pengertian Setiap Tools**

### 🌐 Internet of Things (IoT)
Konsep jaringan yang menghubungkan berbagai perangkat agar dapat saling berkomunikasi dan bertukar data melalui internet.  
Dalam sistem TROPHEUS, IoT memungkinkan integrasi antara sensor DHT22, mikrokontroler ESP32-S3, dan platform ThingsBoard untuk mengirim serta menampilkan data suhu dan kelembapan secara real-time.  

**Komponen utama:**
1. 📡 *Sensor & Aktuator* – membaca kondisi fisik seperti suhu dan kelembapan.  
2. 🧠 *Mikrokontroler (ESP32-S3)* – mengolah data sensor dan mengirimnya ke server.  
3. ☁️ *Platform Cloud (ThingsBoard)* – menyimpan dan menampilkan data sensor dalam dashboard visual.  
4. 🔗 *Protokol MQTT* – komunikasi efisien antarperangkat.  

---

### ⚡ Mikrokontroler ESP32-S3
**ESP32-S3** adalah *System-on-Chip (SoC)* dari **Espressif Systems** yang dirancang untuk aplikasi IoT modern.  
Memiliki performa tinggi, hemat daya, dan mendukung konektivitas **Wi-Fi** serta **Bluetooth Low Energy (BLE)**.  

**Spesifikasi utama:**
- CPU: Xtensa LX7 Dual-Core hingga 240 MHz  
- RAM: 512 KB internal  
- Dukungan *vector instruction* untuk pemrosesan AI  
- Fitur keamanan: *Secure Boot* & *Flash Encryption*  
- Antarmuka: UART, SPI, I2C, PWM, USB OTG  

📌 Pada proyek ini, ESP32-S3 berfungsi membaca data sensor DHT22 dan mengirimkan data ke ThingsBoard Cloud melalui MQTT secara periodik dan real-time.  

---

### 🌡️ Sensor DHT22
**DHT22 (AM2302)** digunakan untuk mengukur suhu dan kelembapan secara digital dengan akurasi tinggi dan konsumsi daya rendah.  

| Parameter | Nilai |
|------------|--------|
| Rentang suhu | -40°C hingga 80°C |
| Akurasi suhu | ±0.5°C |
| Rentang kelembapan | 0–100% RH |
| Akurasi kelembapan | ±2–5% RH |
| Frekuensi sampling | 0.5 Hz (1 data / 2 detik) |

---

### 📡 Protokol MQTT
Protokol komunikasi ringan berbasis *publish/subscribe* yang sangat cocok untuk perangkat IoT dengan bandwidth rendah.  

**Kelebihan:**
- Penggunaan bandwidth efisien  
- Stabil di jaringan lemah  
- Mendukung QoS 0–2  
- Komunikasi asynchronous  

Pada TROPHEUS, MQTT digunakan untuk mengirim data suhu dan kelembapan ke ThingsBoard Cloud serta menangani komunikasi RPC untuk pembaruan firmware OTA.  

---

### ☁️ Platform ThingsBoard Cloud
Platform *open-source IoT* untuk mengelola perangkat, menyimpan data sensor, dan menampilkan dashboard interaktif.  

**Fitur unggulan:**
- Dashboard visual real-time  
- Rule Engine otomatis  
- Autentikasi token perangkat  
- Dukungan TLS/SSL  

---

### 🔧 Firmware Sistem IoT
Firmware dikembangkan menggunakan **C/C++ (Arduino Framework)** dan mengatur:
- Inisialisasi Wi-Fi  
- Pembacaan sensor DHT22  
- Komunikasi MQTT ke ThingsBoard  
- Pembaruan OTA via RPC  

---

## 🧩 **Arsitektur Sistem**
Sistem **TROPHEUS** dirancang dengan arsitektur **open-loop**, di mana perangkat hanya melakukan pengukuran dan pengiriman data tanpa umpan balik otomatis terhadap lingkungan.  

**Diagram Alur Kerja Sistem:**  
