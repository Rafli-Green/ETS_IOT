# PROJECT AKHIR TROPHEUS  
Sistem Monitoring Suhu dan Kelembaban Udara Real-Time dan Terintegrasi untuk Ruang Kelas

---

## Supervisor  
**Ahmad Radhy, S.SI., M.SI**

---

## Anggota Kelompok  
1. Ahmad Rafli Al Adzani — 2042231001  
2. Valencia Christina Setiowardhani — 2042231055  

Program Studi D4 Teknologi Rekayasa Instrumentasi  
Departemen Teknik Instrumentasi  
Fakultas Vokasi  
Institut Teknologi Sepuluh Nopember (ITS)  
Tahun 2025

---

## Deskripsi Proyek  
TROPHEUS merupakan sistem Internet of Things (IoT) berbasis mikrokontroler ESP32-S3 dan sensor DHT22 yang digunakan untuk memantau suhu dan kelembaban udara di ruang kelas Tower 3 ITS secara real-time.  
Sistem ini mengirimkan data melalui protokol MQTT ke platform ThingsBoard Cloud untuk divisualisasikan pada dashboard pemantauan berbasis web.  
Tujuan dari pengembangan sistem ini adalah untuk mendukung kenyamanan termal ruang belajar di Fakultas Vokasi ITS melalui pemantauan suhu dan kelembapan yang terintegrasi.

---

## Latar Belakang  
Kenyamanan termal memiliki pengaruh signifikan terhadap fokus dan produktivitas mahasiswa dalam kegiatan belajar mengajar. Berdasarkan standar **ANSI/ASHRAE 55-2023**, suhu nyaman bagi manusia di ruang indoor berkisar antara 20°C hingga 24°C, sedangkan menurut **Permenkes RI No.1077/Menkes/Per/V/2011**, kelembaban udara yang ideal berada pada rentang 40% hingga 60%.  
Untuk memastikan kondisi ruang kelas tetap dalam batas kenyamanan tersebut, diperlukan sistem pemantauan yang mampu membaca dan mengirimkan data suhu serta kelembapan udara secara aktual dan berkelanjutan.  
TROPHEUS dikembangkan sebagai solusi monitoring berbasis IoT yang sederhana, efisien, dan dapat diakses secara daring melalui platform ThingsBoard.

---

## Tujuan  
1. Merancang sistem monitoring suhu dan kelembapan udara berbasis IoT menggunakan ESP32-S3 dan sensor DHT22.  
2. Mengimplementasikan komunikasi data menggunakan protokol MQTT ke ThingsBoard Cloud.  
3. Menampilkan data suhu dan kelembapan secara real-time pada dashboard pemantauan.  
4. Memberikan dasar bagi pengembangan konsep smart campus pada Fakultas Vokasi ITS.

---

## Komponen Sistem  
| Komponen | Fungsi |
|-----------|---------|
| ESP32-S3 | Mikrokontroler utama yang membaca data sensor dan mengirimkannya ke cloud |
| DHT22 | Sensor digital untuk mengukur suhu dan kelembapan udara |
| ThingsBoard Cloud | Platform berbasis cloud untuk visualisasi data dan pemantauan real-time |
| Protokol MQTT | Jalur komunikasi antara mikrokontroler dan server cloud |
| GNU Plot | Alat bantu untuk analisis hasil pengujian data sistem |

---

## Arsitektur Sistem  
