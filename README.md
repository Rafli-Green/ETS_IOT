# 🌡️ TROPHEUS  
**Sistem Monitoring Suhu dan Kelembaban Udara Real-Time dan Terintegrasi untuk Ruang Kelas**

---

## 👨‍🏫 Supervisor  
**Ahmad Radhy, S.SI., M.SI**

---

## 👥 Anggota Kelompok  
- **Ahmad Rafli Al Adzani** — 2042231001  
- **Valencia Christina Setiowardhani** — 2042231055  

Program Studi D4 Teknologi Rekayasa Instrumentasi  
Departemen Teknik Instrumentasi  
Fakultas Vokasi — Institut Teknologi Sepuluh Nopember (ITS)  
2025  

---

## 📘 Deskripsi Proyek  
**TROPHEUS** adalah sistem **Internet of Things (IoT)** berbasis **ESP32-S3** dan **sensor DHT22** yang dirancang untuk memantau suhu dan kelembaban udara secara **real-time** di ruang kelas Tower 3 ITS.  
Sistem ini mengirimkan data ke **ThingsBoard Cloud** melalui **protokol MQTT**, lalu menampilkannya pada dashboard interaktif berbasis web.

---

## 🧠 Latar Belakang  
Kenyamanan termal berperan penting terhadap fokus dan produktivitas mahasiswa.  
Menurut **ANSI/ASHRAE 55-2023**, suhu nyaman untuk aktivitas belajar berada di rentang **20°C–24°C**, sedangkan **kelembapan ideal** menurut **Permenkes RI No.1077/2011** adalah **40%–60% RH**.  
Melalui sistem **TROPHEUS**, kondisi suhu dan kelembapan ruang kelas dapat dipantau secara kontinu dan akurat guna mendukung kenyamanan belajar di lingkungan Fakultas Vokasi ITS.

---

## 🎯 Tujuan  
- Merancang dan mengimplementasikan **sistem monitoring suhu dan kelembapan** berbasis IoT.  
- Menampilkan data sensor secara **real-time** pada dashboard ThingsBoard Cloud.  
- Menyediakan data termal ruang kelas untuk evaluasi dan pengembangan **Smart Campus System**.

---

## 🧩 Komponen Utama  
| Komponen | Fungsi |
|-----------|---------|
| **ESP32-S3** | Mikrokontroler utama dengan Wi-Fi dan BLE untuk konektivitas IoT |
| **DHT22 (AM2302)** | Sensor suhu dan kelembapan digital dengan akurasi tinggi |
| **ThingsBoard Cloud** | Platform visualisasi data berbasis MQTT |
| **MQTT Protocol** | Protokol komunikasi ringan untuk pengiriman data sensor |
| **GNU Plot** | Analisis hasil pengujian data selama periode monitoring |

---

## ⚙️ Arsitektur Sistem  
