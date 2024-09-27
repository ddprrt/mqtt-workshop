# Linux Days Workshop -- Intro to Rust

Themen:

1. Ownership & Borrowing
2. Modelling Data
3. Error Handling
4. Data Conversions and Auto-Traits

Fokus: Sicherheitsaspekte im Ownership/Borrowing Modell und Stärken des Typsystems beim Erzeugen, Konvertieren und Behandeln von Daten, vor allem über Enums.

## Schritt 1: Pub/Sub einer Message Queue

- Installation der Dependencies über `cargo add`

```toml
[dependencies]
chrono = "0.4.38"
rumqttc = "0.24.0"
serde = { version = "1.0.210", features = ["derive"] }
serde_json = "1.0.128"
```

Erzeugen von MQTT Client und Options

```rust
let mut mqtt_options = MqttOptions::new("linux-days-ed", "test.mosquitto.org", 1883);
mqtt_options.set_keep_alive(Duration::from_secs(5));

let (client, mut connection) = Client::new(mqtt_options, 10);
client
    .subscribe("ldayssensor/data", QoS::AtMostOnce)
    .unwrap();


// ...

for notification in connection.iter() {
    println("{:?}", notification)
}
```

Zu diskutieren:
- `mut` vs immutable, wann braucht man was?
- Tuple Destructuring
- Hinweis auf `unwrap()`

## Schritt 2: Lesen und Publizieren von Sensordaten

Modellierung der Sensordaten

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

Zu diskutieren:
- Auto-Derives
- Celsius vs `f64`
- Hinweis aufs Typsystem und Zero Cost Abstractions


Implementierung von `Display` und `From`

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

Erstellen der Lesefunktion
- Error-Handling



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

## Schritt 3: Lesen der Daten und Senden an die Queue.

Neuen Thread erzeugen

```rust
thread::spawn(move || {

});
```

Zu diskutieren:
- Warum `thread::spawn`
- Warum `move`

Lesen, Serialisieren und Senden der Daten.

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

Zu diskutieren:
- Error Handling verpflichtet! `unwrap`, `expect`, match
- Warum als Referenz bei `to_string`
- Warum `clone` auf Zeile 13


## Schritt 4: Empfangen und Darstellen der Daten

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

Zu diskutieren:
- Ownership Regeln im Funktionskopf
- Iteratoren
- Destructuring der Enums und `if let`
- Wie kommt man von `&packet.payload` zu `&[u8]`

## Schritt 5: Refactoring der `match` Insel

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

Zu diskutieren:
- `Box<dyn Error>` vs Conversions zu `SensorError`

## Schritt 6:

Schlussausbau: Modularisieren, `pub` Schnittstellen. Diskutieren der Datentypen in SensorData. Orphan Rule.
