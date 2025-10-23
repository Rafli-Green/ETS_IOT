## TROPHEUS  
Sistem Monitoring Suhu dan Kelembaban Udara Real-Time dan Terintegrasi untuk Ruang Kelas merupakan proyek berbasis Internet of Things (IoT) yang dikembangkan untuk meningkatkan kualitas kenyamanan termal di lingkungan pendidikan, khususnya pada Tower 3 Fakultas Vokasi Institut Teknologi Sepuluh Nopember (ITS). Sistem ini dirancang untuk memantau suhu dan kelembaban udara secara aktual, kontinu, dan terintegrasi, dengan tujuan mendukung terciptanya ruang belajar yang sehat, efisien, dan produktif bagi mahasiswa maupun dosen.

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
Proyek ini berjudul **TROPHEUS (Temperature and Humidity Real-Time Observation and Processing Hub for Educational Utility Spaces)**, yaitu sebuah sistem berbasis **Internet of Things (IoT)** yang dikembangkan untuk melakukan **pemantauan suhu dan kelembaban udara secara real-time** pada ruang kelas di Tower 3 Fakultas Vokasi ITS.  
Pengembangan sistem ini berangkat dari kebutuhan akan **kenyamanan termal di ruang belajar**, yang terbukti memiliki hubungan langsung dengan tingkat konsentrasi, daya tangkap, dan produktivitas mahasiswa. Berdasarkan standar **ANSI/ASHRAE 55-2023**, suhu nyaman berada pada kisaran **20°C–24°C**, sedangkan **Permenkes RI No.1077/Menkes/Per/V/2011** menetapkan kelembaban ideal antara **40%–60%**.  

Sayangnya, pemantauan kondisi termal di banyak ruang kelas masih dilakukan secara manual, sehingga tidak efisien dan tidak menyediakan data kontinu yang dapat digunakan untuk analisis kenyamanan lingkungan.  
Melalui sistem **TROPHEUS**, data suhu dan kelembapan dapat dikumpulkan secara otomatis menggunakan sensor **DHT22** yang terhubung ke **mikrokontroler ESP32-S3**, kemudian dikirim ke **ThingsBoard Cloud** menggunakan **protokol MQTT** untuk divisualisasikan pada dashboard daring.  

Sistem ini tidak hanya bertujuan untuk memberikan informasi kondisi lingkungan secara aktual, tetapi juga menjadi langkah awal menuju **implementasi Smart Campus** di lingkungan ITS. Data yang dikumpulkan dapat digunakan sebagai dasar pengambilan keputusan terkait pengaturan suhu ruangan, manajemen energi, dan kenyamanan belajar di lingkungan akademik.

---

## Latar Belakang  
Kenyamanan termal merupakan salah satu faktor yang memengaruhi efektivitas proses belajar mengajar di perguruan tinggi. Ruangan yang terlalu panas atau terlalu lembab dapat menurunkan tingkat konsentrasi mahasiswa serta mengganggu efektivitas pembelajaran.  
Dengan meningkatnya fokus terhadap efisiensi energi dan kualitas lingkungan belajar, sistem pemantauan berbasis Internet of Things (IoT) menjadi solusi yang relevan karena mampu menyediakan **data real-time, akurat, dan terintegrasi secara daring**.  

TROPHEUS dirancang sebagai sistem monitoring yang andal dan efisien, dengan memanfaatkan kemampuan ESP32-S3 untuk membaca data dari sensor DHT22 dan mengirimkannya ke platform cloud ThingsBoard melalui protokol MQTT. Sistem ini kemudian menampilkan hasil pemantauan dalam bentuk dashboard interaktif yang mudah diakses oleh dosen, mahasiswa, maupun pihak pengelola fasilitas kampus.

---

## Tujuan  
1. Merancang dan mengimplementasikan sistem pemantauan suhu dan kelembapan ruang kelas berbasis IoT.  
2. Menghubungkan data sensor dengan platform cloud ThingsBoard menggunakan protokol MQTT.  
3. Menyediakan tampilan dashboard interaktif untuk pemantauan data secara real-time.  
4. Mendukung inisiatif pengembangan **Smart Campus ITS** melalui sistem monitoring lingkungan digital.

---

## Komponen Sistem  
| Komponen | Fungsi |
|-----------|---------|
| ESP32-S3 | Mikrokontroler utama dengan konektivitas Wi-Fi untuk komunikasi IoT |
| DHT22 | Sensor digital suhu dan kelembapan dengan akurasi tinggi |
| ThingsBoard Cloud | Platform visualisasi dan manajemen data IoT berbasis cloud |
| MQTT Protocol | Protokol komunikasi ringan untuk pengiriman data telemetri |
| GNU Plot | Perangkat lunak analisis grafik hasil pengujian sistem |

---

## Arsitektur Sistem  
