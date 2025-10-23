# THROPHEUS: Sistem Monitoring Temperature dan Kelembaban Udara Real-Time dan Terintegrasi untuk Ruang Kelas

Perkembangan teknologi digital telah membawa perubahan besar dalam berbagai bidang, termasuk pendidikan. Salah satu inovasi penting yang digunakan di ruang belajar modern adalah **Internet of Things (IoT)** — konsep yang memungkinkan perangkat seperti sensor dan mikrokontroler saling terhubung melalui internet untuk memantau dan mengendalikan kondisi lingkungan secara otomatis dan real-time.  

Dalam konteks ruang kelas, **suhu dan kelembaban** memiliki peran penting terhadap kenyamanan dan konsentrasi belajar siswa. Oleh karena itu, proyek **THROPHEUS** dikembangkan sebagai solusi pemantauan otomatis untuk menjaga kualitas udara di ruang kelas.

---

## Deskripsi Proyek

**THROPHEUS** adalah sistem IoT berbasis **ESP32-S3** dan **sensor DHT22** yang ditulis menggunakan bahasa pemrograman **Rust**. Sistem ini berfungsi untuk memonitor suhu dan kelembaban udara secara real-time, mengirim data ke cloud menggunakan **protokol MQTT**, dan menampilkan hasilnya pada **platform ThingsBoard**.  

Sistem ini juga dilengkapi dengan fitur **Over-The-Air (OTA)** agar firmware dapat diperbarui dari jarak jauh tanpa perlu melakukan flashing manual.  

### Fitur Utama:
- Pemantauan suhu dan kelembaban udara secara real-time.
- Pengiriman data melalui protokol **MQTT**.
- Visualisasi data di **ThingsBoard Dashboard**.
- Dukungan pembaruan firmware **Over-The-Air (OTA)**.
- Kode tertanam aman dan efisien menggunakan **Rust**.

---

## Arsitektur Sistem

