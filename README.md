# TROPHEUS  
Sistem Monitoring Suhu dan Kelembaban Udara Real-Time dan Terintegrasi untuk Ruang Kelas

---

## Deskripsi Umum
Perkembangan teknologi **Internet of Things (IoT)** telah membawa dampak besar dalam berbagai sektor, termasuk bidang pendidikan dan pengelolaan fasilitas kampus. Pemanfaatan IoT memungkinkan integrasi antara sensor, mikrokontroler, dan platform cloud untuk memantau serta mengelola kondisi lingkungan secara otomatis dan real-time.  
Dalam konteks ruang kelas, kenyamanan termal memiliki peran penting terhadap fokus, kesehatan, dan produktivitas mahasiswa. Berdasarkan standar **ANSI/ASHRAE 55-2023**, suhu ideal berada di rentang **20°C–24°C**, sementara menurut **Permenkes RI No.1077/Menkes/Per/V/2011**, kelembapan ideal untuk aktivitas manusia berkisar antara **40%–60%**.  

Untuk memenuhi kebutuhan tersebut, dikembangkanlah **TROPHEUS** (*Temperature and Humidity Real-Time Observation and Processing Hub for Educational Utility Spaces*), yaitu sistem pemantauan suhu dan kelembaban udara berbasis **ESP32-S3** dan **sensor DHT22** yang terintegrasi dengan **ThingsBoard Cloud** menggunakan **protokol MQTT**.  
TROPHEUS dirancang sebagai langkah awal menuju konsep **Smart Campus ITS**, di mana setiap ruang kelas dapat dipantau secara digital untuk mendukung efisiensi energi, kenyamanan belajar, dan pengambilan keputusan berbasis data.

---

## NAMA KELOMPOK 3
**Ahmad Rafli Al Adzani (2042231001)**  
**Valencia Christina Setiowardhani (2042231055)**  
**Ahmad Radhy, S.SI., M.SI (Supervisor)**  

Program Studi D4 Teknologi Rekayasa Instrumentasi  
Departemen Teknik Instrumentasi  
Fakultas Vokasi – Institut Teknologi Sepuluh Nopember (ITS)  
Tahun 2025  

---

## Pengertian Setiap Tools

### Internet of Things (IoT)
**Internet of Things (IoT)** adalah konsep jaringan yang menghubungkan berbagai perangkat agar dapat saling berkomunikasi dan bertukar data melalui internet. Dalam sistem TROPHEUS, IoT memungkinkan integrasi antara sensor DHT22, mikrokontroler ESP32-S3, dan platform ThingsBoard untuk mengirim serta menampilkan data suhu dan kelembapan secara real-time.

Komponen utama dalam sistem IoT:
1. **Perangkat Sensor dan Aktuator** – membaca kondisi fisik seperti suhu dan kelembapan.  
2. **Mikrokontroler (ESP32-S3)** – mengolah data sensor dan mengirimnya ke server.  
3. **Platform Cloud (ThingsBoard)** – menyimpan dan menampilkan data sensor dalam bentuk dashboard visual.  
4. **Protokol MQTT** – jalur komunikasi efisien antarperangkat.

Dalam proyek ini, IoT berfungsi untuk mewujudkan sistem pemantauan lingkungan ruang kelas yang terintegrasi dan mudah diakses.

---

### Mikrokontroler ESP32-S3
**ESP32-S3** merupakan mikrokontroler *System-on-Chip (SoC)* yang dikembangkan oleh **Espressif Systems** untuk kebutuhan IoT modern.  
Keunggulan ESP32-S3 terletak pada performa tinggi, efisiensi daya, dan dukungan konektivitas nirkabel seperti **Wi-Fi** dan **Bluetooth Low Energy (BLE)**.

**Spesifikasi utama ESP32-S3:**
- CPU: Xtensa LX7 Dual-Core hingga 240 MHz  
- RAM: 512 KB SRAM internal  
- Dukungan *vector instruction* untuk pemrosesan AI  
- Fitur keamanan: *Secure Boot* & *Flash Encryption*  
- Antarmuka komunikasi: UART, SPI, I2C, PWM, USB OTG  

Dalam proyek **TROPHEUS**, ESP32-S3 berfungsi sebagai pusat pemrosesan data. Mikrokontroler ini membaca hasil pengukuran suhu dan kelembapan dari sensor DHT22, kemudian mengirimkannya ke platform ThingsBoard Cloud melalui protokol MQTT secara periodik dan real-time.

---

### Sensor DHT22
**DHT22 (AM2302)** adalah sensor digital yang digunakan untuk mengukur suhu dan kelembapan dengan akurasi tinggi serta konsumsi daya rendah.  
Sensor ini menggunakan elemen **NTC thermistor** untuk pengukuran suhu dan **sensor kelembapan kapasitif** untuk mendeteksi kadar uap air di udara.

**Karakteristik utama DHT22:**
| Parameter | Nilai |
|------------|--------|
| Rentang suhu | -40°C hingga 80°C |
| Akurasi suhu | ±0.5°C |
| Rentang kelembapan | 0%–100% RH |
| Akurasi kelembapan | ±2%–5% RH |
| Frekuensi sampling | 0.5 Hz (1 data per 2 detik) |

Sensor DHT22 mengirimkan data digital ke mikrokontroler ESP32-S3 menggunakan komunikasi **single-wire**, sehingga mudah diintegrasikan. Dalam TROPHEUS, DHT22 berperan sebagai sumber data utama yang memastikan setiap perubahan suhu dan kelembapan di ruang kelas dapat dipantau secara akurat dan stabil.

---

### Protokol MQTT
**MQTT (Message Queuing Telemetry Transport)** adalah protokol komunikasi ringan berbasis arsitektur *publish/subscribe* yang sangat cocok untuk aplikasi IoT dengan bandwidth terbatas.  
Protokol ini menggunakan peran **broker** sebagai perantara antara pengirim (publisher) dan penerima (subscriber).

**Kelebihan MQTT:**
- Menggunakan bandwidth rendah dan hemat daya  
- Stabil pada jaringan yang tidak ideal  
- Mendukung tiga tingkat QoS (Quality of Service): 0, 1, dan 2  
- Komunikasi asynchronous sehingga cepat dan efisien  

Dalam sistem **TROPHEUS**, MQTT digunakan untuk:
- Mengirim data suhu dan kelembapan dari ESP32-S3 ke server ThingsBoard Cloud  
- Menjamin keandalan komunikasi data real-time antarperangkat IoT  

---

### Platform ThingsBoard Cloud
**ThingsBoard** adalah platform *open-source IoT* yang berfungsi untuk manajemen perangkat, penyimpanan data sensor, serta visualisasi data secara interaktif.  
Platform ini mendukung berbagai protokol komunikasi seperti **MQTT, HTTP, dan CoAP**, serta memiliki basis data *time-series* yang mampu menyimpan data telemetri jangka panjang.

**Fitur unggulan ThingsBoard Cloud:**
- Dashboard interaktif untuk visualisasi data  
- Rule Engine untuk otomatisasi event  
- Mendukung autentikasi token perangkat  
- Integrasi API untuk pengembangan lanjut  
- Keamanan data melalui enkripsi TLS/SSL  

Pada proyek **TROPHEUS**, ThingsBoard digunakan untuk menampilkan data suhu dan kelembapan secara real-time, memungkinkan pengguna melihat kondisi lingkungan ruang kelas kapan pun dan dari mana pun.

---

### Firmware pada Sistem IoT
Firmware merupakan perangkat lunak tertanam yang mengatur operasi dasar sistem IoT seperti pembacaan sensor, koneksi jaringan, dan komunikasi data.  
Pada **TROPHEUS**, firmware dikembangkan menggunakan **C/C++** berbasis *framework* ESP-IDF, dengan fungsi utama:
- Inisialisasi koneksi Wi-Fi  
- Pembacaan sensor DHT22  
- Komunikasi MQTT dengan ThingsBoard Cloud  
- Validasi data sebelum dikirimkan ke server  

Firmware ini berjalan secara *looping*, membaca data secara berkala, kemudian mengirimkannya dalam format JSON ke ThingsBoard melalui koneksi MQTT yang aman dan efisien.

---

### Arsitektur Sistem
