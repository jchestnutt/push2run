use rumqttc::{Client, Connection, Event, Incoming, MqttOptions, QoS};
use std::str;
use std::time::Duration;

use crate::launcher;

pub struct MqttConfig {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub topic: String,
}

pub fn setup_mqtt(config: MqttConfig) -> (Client, Connection) {
    let mut mqttoptions = MqttOptions::new(config.topic, config.host, config.port as u16);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    mqttoptions.set_credentials(config.username, config.password);
    //mqttoptions.set_transport(Transport::tls(Vec::from("Test CA"), None, None));
    Client::new(mqttoptions, 10)
}

pub fn monitor(mut client: Client, mut connection: Connection, launch_data: &launcher::LaunchData) {
    client.subscribe("push2run", QoS::AtMostOnce).unwrap();
    // Iterate to poll the eventloop for connection progress
    for (_i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let result = str::from_utf8(p.payload.as_ref());
                if let Ok(s) = result {
                    println!("Topic: {}, Payload: {:?}", p.topic, s);
                    launch_data.trigger(s);
                };
            }
            _ => {
                println!("Notification = {:?}", notification);
            }
        }
    }
}
