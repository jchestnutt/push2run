use crate::mqtt_monitor;
use toml::value::Table;
use toml::Value;

pub struct Config {
    pub mqtt_file: String,
    pub launcher_file: String,
}

pub fn read_config(config_value: &Value) -> Config {
    Config {
        mqtt_file: config_value["mqtt"].as_table().unwrap()["file"]
            .as_str()
            .unwrap()
            .to_owned(),
        launcher_file: config_value["launchers"].as_table().unwrap()["file"]
            .as_str()
            .unwrap()
            .to_owned(),
    }
}

pub fn read_mqtt(mqtt_table: &Table) -> mqtt_monitor::MqttConfig {
    let host = mqtt_table["host"].as_str().unwrap();
    let port = match mqtt_table.get("port") {
        None => 1883,
        Some(v) => v.as_integer().unwrap(),
    };
    let user = mqtt_table["username"].as_str().unwrap();
    let password = mqtt_table["password"].as_str().unwrap();
    let topic = mqtt_table["topic"].as_str().unwrap();

    mqtt_monitor::MqttConfig {
        host: host.to_owned(),
        port: port as i32,
        username: user.to_owned(),
        password: password.to_owned(),
        topic: topic.to_owned(),
    }
}
