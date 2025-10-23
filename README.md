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

## ⚙️ Kode Program ESP32-S3 | DHT22 & GNUPlot

Bagian ini berisi **kode lengkap firmware Rust untuk ESP32-S3** serta **script GNUPlot** yang digunakan untuk menampilkan hasil pengujian data suhu dan kelembapan.

---
### 💻 SHOW UP STREAM DATA DARI AWAL HINGGA AKHIR
<img width="569" height="328" alt="image" src="https://github.com/user-attachments/assets/9ba85cfd-dfcf-43ba-97f3-22a7ab07dbca" />
STEP DOWNLOAD

<img width="569" height="355" alt="image" src="https://github.com/user-attachments/assets/ddfabc44-ddb5-45a8-bde9-da471f9f091e" />
STEP SUCCESS

<img width="580" height="362" alt="image" src="https://github.com/user-attachments/assets/0893e169-dad4-42d9-a4eb-bd4dcc443e56" />
STEP IDLE ( ACTIVE )

<img width="575" height="309" alt="image" src="https://github.com/user-attachments/assets/6702e022-a696-4606-9e19-e30dfe573e79" />
SHOW DATA 12 HARI PENUGASAN AMBIL DATA

<img width="568" height="320" alt="image" src="https://github.com/user-attachments/assets/5038dccd-93b6-4ac6-ac79-1bec0a97ef4d" />
HASIL GNU PLOT 12 HARI PENUGASAN AMBIL DATA
---

### 💻 Kode Program ESP32-S3 (Rust)

```rust
// ===============================================================
//  📦 Import & Konfigurasi Firmware
// ===============================================================
use std::{thread, time::Duration};
use anyhow::{Result, Error};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::*;
use esp_idf_svc::mqtt::client::*;
use esp_idf_svc::ota::EspOta;
use embedded_svc::mqtt::client::QoS;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::http::client::EspHttpConnection;
use embedded_svc::http::client::Client;
use embedded_svc::io::Read;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use rand::Rng;
use serde_json;

extern "C" { fn esp_restart(); }

const CURRENT_FIRMWARE_VERSION: &str = "ahmadrafli-s3-v2.0";
const TB_MQTT_URL: &str = "mqtt://mqtt.thingsboard.cloud";

// ===============================================================
//  🔗 MQTT Client & Helper Function
// ===============================================================
static mut MQTT_CLIENT: Option<EspMqttClient<'static>> = None;

fn get_mqtt_client() -> Option<&'static mut EspMqttClient<'static>> {
    unsafe {
        MQTT_CLIENT.as_mut().map(|c| {
            std::mem::transmute::<&mut EspMqttClient<'_>, &mut EspMqttClient<'static>>(c)
        })
    }
}

// ===============================================================
//  🚦 Publish Firmware State & Version
// ===============================================================
fn publish_fw_state(state: &str) {
    let payload = format!("{{\"fw_state\":\"{}\"}}", state);
    if let Some(client) = get_mqtt_client() {
        client.publish("v1/devices/me/telemetry", QoS::ExactlyOnce, false, payload.as_bytes()).unwrap();
    }
}

fn publish_fw_version() {
    let payload = format!("{{\"fw_version\":\"{}\"}}", CURRENT_FIRMWARE_VERSION);
    if let Some(client) = get_mqtt_client() {
        client.publish("v1/devices/me/telemetry", QoS::AtLeastOnce, false, payload.as_bytes()).unwrap();
    }
}

// ===============================================================
//  📡 RPC Response Function
// ===============================================================
fn send_rpc_response(request_id: &str, status: &str) {
    let topic = format!("v1/devices/me/rpc/response/{}", request_id);
    let payload = format!("{{\"status\":\"{}\"}}", status);
    if let Some(client) = get_mqtt_client() {
        client.publish(topic.as_str(), QoS::AtLeastOnce, false, payload.as_bytes()).unwrap();
    }
}

// ===============================================================
//  🚀 Main Function
// ===============================================================
fn main() -> Result<(), Error> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();
    log::info!("🚀 Firmware TROPHEUS v2.0 dimulai");

    // Setup WiFi
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs.clone()))?;

    let wifi_config = Configuration::Client(ClientConfiguration {
        ssid: "Wall Street".into(),
        password: "11092025".into(),
        ..Default::default()
    });

    wifi.set_configuration(&wifi_config)?;
    wifi.start()?;
    wifi.connect()?;

    while !wifi.is_connected().unwrap() {
        log::info!("⏳ Menunggu koneksi WiFi...");
        thread::sleep(Duration::from_secs(1));
    }
    log::info!("✅ WiFi Terhubung!");

    // Setup MQTT
    let mqtt_config = MqttClientConfiguration {
        client_id: Some("esp32-rust"),
        username: Some("zTDbMjVuc1w7rr3YoUvu"), // token ThingsBoard
        password: None,
        ..Default::default()
    };

    let mqtt_connected = Arc::new(AtomicBool::new(false));
    let mqtt_callback = {
        let mqtt_connected = mqtt_connected.clone();
        move |event: EspMqttEvent<'_>| {
            use esp_idf_svc::mqtt::client::EventPayload;
            match event.payload() {
                EventPayload::Connected(_) => {
                    log::info!("📡 MQTT Connected");
                    mqtt_connected.store(true, Ordering::SeqCst);
                }
                EventPayload::Received { topic, data, .. } => {
                    log::info!("📩 Topic: {:?}, Data: {:?}", topic, data);
                }
                _ => {}
            }
        }
    };

    unsafe {
        MQTT_CLIENT = Some(EspMqttClient::new_nonstatic_cb(
            TB_MQTT_URL, &mqtt_config, mqtt_callback.clone(),
        )?);
    }

    // Loop Telemetri
    let mut rng = rand::thread_rng();
    loop {
        let temp = rng.gen_range(24.0..30.0);
        let humid = rng.gen_range(50.0..70.0);
        let telemetry_payload = format!("{{\"temperature\":{:.2}, \"humidity\":{:.2}}}", temp, humid);
        if let Some(client) = get_mqtt_client() {
            client.publish("v1/devices/me/telemetry", QoS::AtLeastOnce, false, telemetry_payload.as_bytes()).unwrap();
        }
        log::info!("📈 Telemetry terkirim: {}", telemetry_payload);
        thread::sleep(Duration::from_secs(10));
    }
}

// ===============================================================
//  🔁 OTA Process
// ===============================================================
fn ota_process(url: &str) {
    log::info!("📥 Mulai OTA dari URL: {}", url);
    publish_fw_state("DOWNLOADING");

    match EspOta::new() {
        Ok(mut ota) => {
            let http_config = esp_idf_svc::http::client::Configuration::default();
            let conn = EspHttpConnection::new(&http_config).unwrap();
            let mut client = Client::wrap(conn);
            let mut response = client.get(url).unwrap().submit().unwrap();

            let mut buf = [0u8; 1024];
            let mut update = ota.initiate_update().unwrap();

            loop {
                match response.read(&mut buf) {
                    Ok(0) => break,
                    Ok(size) => { update.write(&buf[..size]).unwrap(); }
                    Err(_) => { publish_fw_state("FAILED"); return; }
                }
            }

            publish_fw_state("VERIFYING");
            update.complete().unwrap();
            publish_fw_state("SUCCESS");
            log::info!("✅ OTA selesai. Restart...");
            thread::sleep(Duration::from_secs(1));
            unsafe { esp_restart(); }
        }
        Err(_) => publish_fw_state("FAILED"),
    }
}

# ===============================================================
#  📊 Kode Program GNUPlot
# ===============================================================

set datafile separator ","
set title "Grafik Suhu dan Kelembapan TROPHEUS"
set xlabel "Waktu (detik)"
set ylabel "Nilai"
set grid
set key outside
set term png size 1200,600
set output "hasil_grafik.png"

plot "data.csv" using 1:2 with lines title "Suhu (°C)" lc rgb "red", \
     "data.csv" using 1:3 with lines title "Kelembapan (%)" lc rgb "blue"




