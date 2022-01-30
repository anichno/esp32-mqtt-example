use std::sync::Arc;

use anyhow::Result;
use embedded_hal::prelude::*;
use embedded_svc::{
    mqtt::client::{Client, Connection, Publish},
    wifi::{self, Wifi},
};
use esp_idf_svc::{mqtt, netif, nvs::EspDefaultNvs, sysloop, wifi::EspWifi};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut wifi = EspWifi::new(
        Arc::new(netif::EspNetifStack::new()?),
        Arc::new(sysloop::EspSysLoopStack::new()?),
        Arc::new(EspDefaultNvs::new()?),
    )?;
    wifi.set_configuration(&wifi::Configuration::Client(wifi::ClientConfiguration {
        ssid: env!("WIFI_SSID").into(),
        password: env!("WIFI_PASS").into(),
        ..Default::default()
    }))?;

    let mqtt_config = mqtt::client::MqttClientConfiguration {
        client_id: Some("esp32"),
        ..Default::default()
    };

    let (mut mqtt_client, mut mqtt_conn) =
        mqtt::client::EspMqttClient::new(env!("MQTT_SERVER"), &mqtt_config)?;

    info!("Connected");

    std::thread::spawn(move || {
        info!("mqtt loop started");
        while let Some(msg) = mqtt_conn.next() {
            match msg {
                Ok(msg) => println!("MQTT: {:?}", msg),
                Err(e) => println!("MQTT err: {:?}", e),
            }
        }
        info!("mqtt loop ended");
    });

    let mut rtos_delay = esp_idf_hal::delay::FreeRtos;

    mqtt_client.publish(
        "status",
        embedded_svc::mqtt::client::QoS::AtMostOnce,
        false,
        "esp32".as_bytes(),
    )?;

    mqtt_client.subscribe(
        "commands/esp32",
        embedded_svc::mqtt::client::QoS::AtMostOnce,
    )?;
    info!("Subscribed");

    loop {
        rtos_delay.delay_ms(100_u32);
    }
}
