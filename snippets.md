```rust
let mut mqtt_options = MqttOptions::new("linux-days-ed", "test.mosquitto.org", 1883);
mqtt_options.set_keep_alive(Duration::from_secs(5));

let (client, mut connection) = Client::new(mqtt_options, 10);
client
    .subscribe("ldayssensor/data", QoS::AtMostOnce)
    .unwrap();


// ...

for notification in connection.iter() {
    println!("{:?}", notification)
}
```


```rust
#[derive(Serialize, Deserialize, Debug)]
struct Celsius(f64);


#[derive(Serialize, Deserialize, Debug)]
struct SensorData {
    temperature: Celsius,
    humidity: f32,
    timestamp: SystemTime,
}
```


```rust
impl From<f64> for Celsius {
    fn from(f: f64) -> Self {
        Celsius(f)
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°C", self.0)
    }
}
```

```rust
#[derive(Debug)]
pub struct SensorError {
    message: String,
}

impl SensorError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::error::Error for SensorError {}

impl std::fmt::Display for SensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SensorError: {}", self.message)
    }
}

fn read_sensor_data() -> Result<SensorData, SensorError> {
    Ok(SensorData {
        temperature: Celsius(25.0),
        humidity: 60.0,
        timestamp: SystemTime::now(),
    })
}
```

```rust
thread::spawn(move || {

});
```


```rust
let sensor_data = read_sensor_data();
let sensor_data = match sensor_data {
    Ok(sensor_data) => sensor_data,
    Err(err) => {
        eprintln!("Sensor Data Error: {}", err);
        continue;
    }
};

let payload = serde_json::to_string(&sensor_data);
let payload = match payload {
    Ok(payload) => payload,
    Err(err) => {
        eprintln!("Serde Error: {}", err);
        continue;
    }
};

match client.publish("ldayssensor/data", QoS::AtLeastOnce, false, payload.clone()) {
    Ok(_) => {
        println!("Published sensor data {i}: {}", payload);
    }
    Err(err) => {
        eprintln!("Send Error: {:?}", err);
    }
}
```

```rust
fn receive_notifications(connection: &mut rumqttc::Connection) {
    for notification in connection.iter() {
        match notification {
            Ok(notification) => {
                if let Event::Incoming(Packet::Publish(packet)) = notification {
                    let payload: SensorData = serde_json::from_slice(&packet.payload).unwrap();
                    print_temperature(&payload.temperature.into());
                }
            }
            Err(err) => {
                eprintln!("Notification Error: {:?}", err);
            }
        }
    }
}
```

```rust
impl From<serde_json::Error> for SensorError {
    fn from(err: serde_json::Error) -> Self {
        Self::new(format!("Serde Error: {}", err))
    }
}

impl From<rumqttc::ClientError> for SensorError {
    fn from(err: rumqttc::ClientError) -> Self {
        Self::new(format!("MQTT Error: {}", err))
    }
}

fn retrieve_and_send_sensor_data(client: &Client) -> Result<(), SensorError> {
    let sensor_data = read_sensor_data()?;
    let payload = serde_json::to_string(&sensor_data)?;
    client.publish("ldayssensor/data", QoS::AtLeastOnce, false, payload)?;
    Ok(())
}
```
