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
Sistem TROPHEUS dirancang dengan arsitektur **open-loop**, di mana perangkat hanya melakukan pengukuran dan pengiriman data tanpa proses umpan balik otomatis terhadap kondisi lingkungan. Data dari sensor dikirimkan secara kontinu melalui jaringan Wi-Fi menuju broker MQTT, kemudian diteruskan ke platform ThingsBoard Cloud untuk divisualisasikan dalam bentuk grafik dan indikator real-time.  

Diagram alur kerja sistem:

---

## 3.8 Kode Program ESP32-S3 | DHT22 & Kode Program GNUPlot

### Kode Program ESP32-S3
Bagian ini memuat program utama untuk **mikrokontroler ESP32-S3** yang terhubung dengan **sensor DHT22** serta berkomunikasi dengan **ThingsBoard Cloud** melalui **protokol MQTT**.  
Kode berikut ditulis menggunakan **bahasa C/C++** dengan *framework* **Arduino-ESP32**, yang dapat dijalankan melalui **Arduino IDE** atau **PlatformIO**.  

```cpp
// ================================================================
// TROPHEUS: ESP32-S3 + DHT22 + MQTT + ThingsBoard Cloud
// ================================================================
// Fitur: Pembacaan sensor suhu & kelembapan, koneksi Wi-Fi, publish
// telemetri ke ThingsBoard, respon RPC untuk OTA, dan ArduinoOTA.
// ================================================================

#include <WiFi.h>
#include <PubSubClient.h>
#include <DHT.h>
#include <ArduinoJson.h>
#include <HTTPClient.h>
#include <HTTPUpdate.h>
#include <ArduinoOTA.h>

// ---------- Konfigurasi Wi-Fi & ThingsBoard ----------
const char* WIFI_SSID = "Nama_WiFi";
const char* WIFI_PASS = "Password_WiFi";
const char* THINGSBOARD_SERVER = "demo.thingsboard.io";
const int   THINGSBOARD_PORT = 1883;
const char* ACCESS_TOKEN = "Token_Device_ThingsBoard";

// ---------- Konfigurasi Sensor DHT22 ----------
#define DHTPIN 4
#define DHTTYPE DHT22
DHT dht(DHTPIN, DHTTYPE);

// ---------- Variabel Global ----------
WiFiClient espClient;
PubSubClient mqttClient(espClient);
unsigned long lastSend = 0;
const unsigned long interval = 10000; // kirim tiap 10 detik

// ---------- Informasi Firmware ----------
const char* FIRMWARE_NAME = "TROPHEUS-ESP32S3";
const char* FIRMWARE_VERSION = "1.0.0";

// ---------- Inisialisasi Wi-Fi ----------
void setupWiFi() {
  Serial.printf("Menghubungkan ke Wi-Fi: %s\n", WIFI_SSID);
  WiFi.begin(WIFI_SSID, WIFI_PASS);
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.printf("\nTerhubung! IP Address: %s\n", WiFi.localIP().toString().c_str());
}

// ---------- Callback MQTT ----------
void callback(char* topic, byte* payload, unsigned int length) {
  String data;
  for (unsigned int i = 0; i < length; i++) data += (char)payload[i];
  Serial.printf("Pesan diterima [%s]: %s\n", topic, data.c_str());

  // Menangani RPC untuk OTA
  if (String(topic).startsWith("v1/devices/me/rpc/request/")) {
    StaticJsonDocument<512> doc;
    if (deserializeJson(doc, data)) return;
    const char* method = doc["method"];
    if (method && String(method) == "ota_update") {
      const char* url = doc["params"]["url"];
      if (url && strlen(url) > 0) {
        Serial.printf("Menjalankan OTA dari URL: %s\n", url);
        t_httpUpdate_return ret = httpUpdate.update(espClient, url);
        switch (ret) {
          case HTTP_UPDATE_FAILED:
            Serial.println("Gagal melakukan OTA.");
            break;
          case HTTP_UPDATE_NO_UPDATES:
            Serial.println("Tidak ada pembaruan baru.");
            break;
          case HTTP_UPDATE_OK:
            Serial.println("Pembaruan berhasil, sistem akan restart.");
            break;
        }
      }
    }
  }
}

// ---------- Koneksi ke MQTT ----------
void reconnectMQTT() {
  while (!mqttClient.connected()) {
    Serial.print("Menghubungkan ke MQTT...");
    if (mqttClient.connect("TROPHEUS_Client", ACCESS_TOKEN, NULL)) {
      Serial.println("Terhubung ke broker MQTT.");
      mqttClient.subscribe("v1/devices/me/rpc/request/+");
    } else {
      Serial.printf("Gagal (rc=%d), coba lagi dalam 5 detik.\n", mqttClient.state());
      delay(5000);
    }
  }
}

// ---------- Publish Telemetri ----------
void publishTelemetry(float temp, float hum) {
  StaticJsonDocument<128> doc;
  doc["temperature"] = temp;
  doc["humidity"] = hum;
  char buffer[128];
  size_t n = serializeJson(doc, buffer);
  mqttClient.publish("v1/devices/me/telemetry", buffer, n);
  Serial.printf("Data terkirim -> Suhu: %.2f°C | Kelembapan: %.2f%%\n", temp, hum);
}

// ---------- Setup ----------
void setup() {
  Serial.begin(115200);
  delay(1000);

  dht.begin();
  setupWiFi();

  mqttClient.setServer(THINGSBOARD_SERVER, THINGSBOARD_PORT);
  mqttClient.setCallback(callback);

  ArduinoOTA.setHostname("TROPHEUS-ESP32S3");
  ArduinoOTA.begin();
  Serial.println("Sistem siap.");
}

// ---------- Loop ----------
void loop() {
  ArduinoOTA.handle();

  if (WiFi.status() != WL_CONNECTED) setupWiFi();
  if (!mqttClient.connected()) reconnectMQTT();

  mqttClient.loop();

  unsigned long now = millis();
  if (now - lastSend > interval) {
    lastSend = now;
    float h = dht.readHumidity();
    float t = dht.readTemperature();
    if (isnan(h) || isnan(t)) {
      Serial.println("Gagal membaca sensor DHT22!");
      return;
    }
    publishTelemetry(t, h);
  }
}

# plot.gnuplot
set datafile separator ","
set xdata time
set timefmt "%Y-%m-%d %H:%M:%S"
set format x "%H:%M\n%d-%m"
set grid
set title "Grafik Monitoring Suhu dan Kelembapan - TROPHEUS"
set xlabel "Waktu Pengamatan"
set ylabel "Nilai (°C / %RH)"
set key outside
plot "data_log.csv" using 1:2 with lines title "Temperature (°C)", \
     "data_log.csv" using 1:3 with lines title "Humidity (%RH)"

## Tampilan Streaming Data
Berikut contoh tampilan streaming data pada ThingsBoard Cloud:

![Streaming Data - Dashboard ThingsBoard](assets/streaming_dashboard.png)

**Keterangan:**
- Panel kiri menampilkan grafik suhu (°C) dengan interval pembaruan 10 detik.  
- Panel kanan menampilkan grafik kelembapan udara (%RH).  
- Indikator status perangkat menunjukkan kondisi koneksi MQTT dan uptime sistem.

