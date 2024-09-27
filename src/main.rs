use std::{
    error::Error,
    fmt::Display,
    thread,
    time::{Duration, SystemTime},
};

use rumqttc::{Client, ClientError, Event, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Celsius(f32);

impl From<f32> for Celsius {
    fn from(value: f32) -> Self {
        Celsius(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SensorData {
    temperature: Celsius,
    humidity: f32,
    timestamp: SystemTime,
}

#[derive(Debug)]
struct SensorError {
    message: String,
}

impl Error for SensorError {}

impl Display for SensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A sensor error happened: {}", self.message)
    }
}

impl SensorError {
    fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl From<serde_json::Error> for SensorError {
    fn from(value: serde_json::Error) -> Self {
        SensorError::new(value.to_string())
    }
}

impl From<ClientError> for SensorError {
    fn from(value: ClientError) -> Self {
        SensorError::new(value.to_string())
    }
}

fn read_sensor_data() -> Result<SensorData, SensorError> {
    Ok(SensorData {
        temperature: 20.0.into(),
        humidity: 45.0,
        timestamp: SystemTime::now(),
    })
}

fn main() {
    let mut mqtt_options = MqttOptions::new("linux-days-4188", "test.mosquitto.org", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqtt_options, 10);
    client
        .subscribe("linuxdays/workshop", QoS::AtMostOnce)
        .expect("I expect my queue to subscribe");

    thread::spawn(move || loop {
        if let Err(err) = read_and_publish(&client) {
            println!("Sorry, did not work, again in 4 secs, {}", err);
        }

        thread::sleep(Duration::from_secs(4));
    });

    for (_i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(notification) => {
                if let Event::Incoming(Packet::Publish(packet)) = notification {
                    let payload: SensorData = serde_json::from_slice(&packet.payload).unwrap();
                    println!("{:?}", payload);
                }
            }
            Err(err) => {
                eprintln!("Notification Error: {:?}", err);
            }
        }
    }
}

fn read_and_publish(client: &Client) -> Result<(), SensorError> {
    let sensor_data = read_sensor_data()?;
    let payload = serde_json::to_string(&sensor_data)?;
    client.publish("linuxdays/workshop", QoS::AtLeastOnce, false, payload)?;
    Ok(())
}
