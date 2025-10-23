// Dependencies
use std::{thread, time::Duration};
use anyhow::{Result, Error};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::*;
use esp_idf_svc::mqtt::client::*;
use esp_idf_svc::ota::EspOta;
use embedded_svc::mqtt::client::QoS;
use heapless::String;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::http::client::EspHttpConnection;
use embedded_svc::http::client::Client;
use embedded_svc::io::Read;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use rand::Rng;
use serde_json;

extern "C" {
    fn esp_restart();
}

// 📌 KONFIGURASI FW: Ubah ke versi BARU. Ini adalah versi firmware v2.0 yang akan di-OTA.
const CURRENT_FIRMWARE_VERSION: &str = "ahmadrafli-s3-v2.0";

// ✅ PERUBAHAN: Alamat IP server diubah ke ThingsBoard Cloud.
const TB_MQTT_URL: &str = "mqtt://mqtt.thingsboard.cloud";

// =========================================================================================
static mut MQTT_CLIENT: Option<EspMqttClient<'static>> = None;

fn get_mqtt_client() -> Option<&'static mut EspMqttClient<'static>> {
    unsafe {
        MQTT_CLIENT.as_mut().map(|c| {
            std::mem::transmute::<&mut EspMqttClient<'_>, &mut EspMqttClient<'static>>(c)
        })
    }
}

// Fungsi untuk mengirim telemetry fw_state
fn publish_fw_state(state: &str) {
    let payload = format!("{{\"fw_state\":\"{}\"}}", state);
    log::info!("➡️ Mengirim telemetry fw_state: {}", payload);

    if let Some(client) = get_mqtt_client() {
        // MENGGUNAKAN QoS::ExactlyOnce (QoS 2)
        if let Err(e) = client.publish(
            "v1/devices/me/telemetry",
            QoS::ExactlyOnce,
            false,
            payload.as_bytes(),
        ) {
            log::error!("⚠️ Gagal kirim fw_state {}: {:?}", state, e);
        }
    } else {
        log::error!("⚠️ MQTT client belum siap untuk kirim fw_state {}", state);
    }
}

// Mengirim versi firmware saat ini (CRUCIAL UNTUK OTA)
fn publish_fw_version() {
    let payload = format!("{{\"fw_version\":\"{}\"}}", CURRENT_FIRMWARE_VERSION);
    log::info!("➡️ Mengirim Current FW Version: {}", payload);

    if let Some(client) = get_mqtt_client() {
        if let Err(e) = client.publish(
            "v1/devices/me/telemetry",
            QoS::AtLeastOnce,
            false,
            payload.as_bytes(),
        ) {
            log::error!("⚠️ Gagal kirim fw_version: {:?}", e);
        }
    } else {
        log::error!("⚠️ MQTT client belum siap untuk kirim fw_version");
    }
}

// Fungsi untuk mengirim RPC response ke ThingsBoard
fn send_rpc_response(request_id: &str, status: &str) {
    let topic = format!("v1/devices/me/rpc/response/{}", request_id);
    log::info!("➡️ Mengirim RPC response ke: {}", topic);

    let payload = format!("{{\"status\":\"{}\"}}", status);

    if let Some(client) = get_mqtt_client() {
        if let Err(e) = client.publish(
            topic.as_str(),
            QoS::AtLeastOnce,
            false,
            payload.as_bytes(),
        ) {
            log::error!("⚠️ Gagal kirim RPC response: {:?}", e);
        }
    } else {
        log::error!("⚠️ MQTT client belum siap untuk kirim RPC response");
    }
}
// =========================================================================================

fn main() -> Result<(), Error> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();
    log::info!("🚀 Program dimulai, Versi FW: {} - 🔥 V2.0 FIRMWARE AKTIF!", CURRENT_FIRMWARE_VERSION);

    // --- WiFi Setup ---
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs.clone()))?;

    let mut ssid: String<32> = String::new();
    ssid.push_str("Wall Street").unwrap();

    let mut pass: String<64> = String::new();
    pass.push_str("11092025").unwrap();

    let wifi_config = Configuration::Client(ClientConfiguration {
        ssid,
        password: pass,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_config)?;
    wifi.start()?;
    wifi.connect()?;

    while !wifi.is_connected().unwrap() {
        log::info!("⏳ Menunggu koneksi WiFi...");
        thread::sleep(Duration::from_secs(1));
    }
    log::info!("✅ Terhubung ke WiFi!");

    let _wifi = Box::leak(Box::new(wifi));
    let _sysloop = Box::leak(Box::new(sysloop));
    let _nvs = Box::leak(Box::new(nvs));

    thread::sleep(Duration::from_secs(3));

    // --- MQTT ThingsBoard ---
    // 📌 PENTING: Pastikan Anda mengganti `username` dengan Access Token dari perangkat Anda di ThingsBoard Cloud.
    let mqtt_config = MqttClientConfiguration {
        client_id: Some("esp32-rust"),
        username: Some("zTDbMjVuc1w7rr3YoUvu"), // <-- GANTI DENGAN ACCESS TOKEN BARU ANDA
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
                    log::info!("📡 MQTT connected");
                    mqtt_connected.store(true, Ordering::SeqCst);
                }
                EventPayload::Received { topic, data, .. } => {
                    let payload_str = std::str::from_utf8(data).unwrap_or("");
                    log::info!("📩 Payload diterima. Topic: {:?}, Data: {}", topic, payload_str);
                    
                    if let Some(topic_str) = topic {
                        if topic_str.starts_with("v1/devices/me/rpc/request/") {
                            let parts: Vec<&str> = topic_str.split('/').collect();
                            if let Some(request_id) = parts.last() {
                                log::info!("✅ Menerima RPC request_id: {}", request_id);

                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(payload_str) {
                                    let mut ota_url = None;

                                    if let Some(params) = json.get("params") {
                                        if let Some(url) = params.get("ota_url").and_then(|u| u.as_str()) {
                                            ota_url = Some(url);
                                        }
                                    }
                                    
                                    if let Some(url) = ota_url {
                                        log::info!("⚡ Dapat OTA URL dari RPC: {}", url);
                                        
                                        send_rpc_response(request_id, "success");
                                        
                                        ota_process(url);
                                        return;
                                    } else {
                                        log::warn!("⚠️ Payload RPC diterima, tetapi 'ota_url' tidak ditemukan di dalam 'params'.");
                                    }
                                    
                                } else {
                                    log::error!("⚠️ Gagal mem-parse JSON payload RPC.");
                                }
                                
                                send_rpc_response(request_id, "failure");
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    };

    let client = loop {
        let res = unsafe {
            EspMqttClient::new_nonstatic_cb(
                TB_MQTT_URL,
                &mqtt_config,
                mqtt_callback.clone(),
            )
        };

        match res {
            Ok(c) => {
                unsafe { MQTT_CLIENT = Some(c) };

                if let Some(c_ref) = get_mqtt_client() {
                    while !mqtt_connected.load(Ordering::SeqCst) {
                        log::info!("⏳ Menunggu MQTT connect...");
                        thread::sleep(Duration::from_millis(500));
                    }
                    log::info!("📡 MQTT Connected!");
                    
                    c_ref.subscribe("v1/devices/me/rpc/request/+", QoS::AtLeastOnce).unwrap();
                    
                    // LAPORAN INI KRUSIAL UNTUK OTA
                    publish_fw_version();
                    publish_fw_state("IDLE");
                    
                    break c_ref;
                } else {
                    log::error!("⚠️ Gagal mendapatkan referensi client setelah koneksi.");
                    thread::sleep(Duration::from_secs(5));
                    continue;
                }
            }
            Err(e) => {
                log::error!("⚠️ MQTT connect gagal: {:?}", e);
                thread::sleep(Duration::from_secs(5));
            }
        }
    };
    
    let mut rng = rand::thread_rng();
    let mut telemetry_counter = 0;
    
    const TELEMETRY_INTERVAL_SECS: u64 = 10;
    const POLLING_INTERVAL_SECS: u64 = 1;
    const POLLING_COUNT_MAX: u64 = TELEMETRY_INTERVAL_SECS / POLLING_INTERVAL_SECS;

    loop {
        if telemetry_counter >= POLLING_COUNT_MAX {
            let temp = rng.gen_range(20.0..30.0);
            let humid = rng.gen_range(50.0..70.0);
            
            let telemetry_payload = format!(
                "{{\"temperature\":{:.2}, \"humidity\":{:.2}}}",
                temp, humid
            );

            if let Err(e) = client.publish(
                "v1/devices/me/telemetry",
                QoS::AtLeastOnce,
                false,
                telemetry_payload.as_bytes()
            ) {
               log::error!("⚠️ Gagal kirim telemetry: {:?}", e);
            } else {
                log::info!("Sent telemetry: {}", telemetry_payload);
            }
            
            telemetry_counter = 0;
        }
        
        thread::sleep(Duration::from_secs(POLLING_INTERVAL_SECS));
        telemetry_counter += 1;
    }
}

// --- OTA process ---
fn ota_process(url: &str) {
    log::info!("📥 Mulai OTA dari URL: {}", url);
    publish_fw_state("DOWNLOADING");

    match EspOta::new() {
        Ok(mut ota) => {
            let http_config = esp_idf_svc::http::client::Configuration {
                ..Default::default()
            };

            let conn = match EspHttpConnection::new(&http_config) {
                Ok(c) => c,
                Err(e) => {
                    log::error!("⚠️ Gagal buat koneksi HTTP: {:?}", e);
                    publish_fw_state("FAILED");
                    return;
                }
            };

            let mut client = Client::wrap(conn);
            let request = match client.get(url) {
                Ok(r) => r,
                Err(e) => {
                    log::error!("⚠️ Gagal buat HTTP GET: {:?}", e);
                    publish_fw_state("FAILED");
                    return;
                }
            };

            let mut response = match request.submit() {
                Ok(r) => r,
                Err(e) => {
                    log::error!("⚠️ Gagal submit request: {:?}", e);
                    publish_fw_state("FAILED");
                    return;
                }
            };
            
            if response.status() < 200 || response.status() >= 300 {
                log::error!("⚠️ HTTP request gagal. Status code: {}", response.status());
                publish_fw_state("FAILED");
                return;
            }

            let mut buf = [0u8; 1024];
            let mut update = match ota.initiate_update() {
                Ok(u) => u,
                Err(e) => {
                    log::error!("⚠️ Gagal init OTA: {:?}", e);
                    publish_fw_state("FAILED");
                    return;
                }
            };

            loop {
                match response.read(&mut buf) {
                    Ok(0) => break,
                    Ok(size) => {
                        if let Err(e) = update.write(&buf[..size]) {
                            log::error!("⚠️ Gagal tulis OTA: {:?}", e);
                            publish_fw_state("FAILED");
                            return;
                        }
                    }
                    Err(e) => {
                        log::error!("⚠️ HTTP read error: {:?}", e);
                        publish_fw_state("FAILED");
                        return;
                    }
                }
            }
            
            publish_fw_state("VERIFYING");

            if let Err(e) = update.complete() {
                log::error!("⚠️ OTA complete error: {:?}", e);
                publish_fw_state("FAILED");
                return;
            }

            log::info!("✅ OTA selesai, restart...");
            publish_fw_state("SUCCESS");
            
            // ✅ FIX: Jeda 1 detik (Cukup untuk QoS 2)
            thread::sleep(Duration::from_secs(1));
            
            unsafe { esp_restart(); }
        }
        Err(e) => {
            log::error!("⚠️ Gagal init OTA: {:?}", e);
            publish_fw_state("FAILED");
        }
    }
}

