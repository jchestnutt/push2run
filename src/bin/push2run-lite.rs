use flexi_logger::{AdaptiveFormat, Duplicate, FileSpec, Logger};
use log;
use push2run::loader;
use push2run::mqtt_monitor::{monitor, setup_mqtt};
use push2run::read_config;
use std::fs;
use toml::Value;

fn main() {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default())
        .append()
        .duplicate_to_stderr(Duplicate::Info)
        .adaptive_format_for_stderr(AdaptiveFormat::Default)
        .append()
        .start()
        .unwrap();

    let config_value: Value = fs::read_to_string("config.ini")
        .expect("Could not open config file.")
        .parse()
        .unwrap();
    let config = read_config::read_config(&config_value);
    let launch_data = loader::load_data(&config.launcher_file).unwrap();
    //info!("{:?}", config.mqtt_file);
    let mqtt_value: Value = fs::read_to_string(config.mqtt_file)
        .expect("Could not open mqtt file.")
        .parse()
        .unwrap();
    let mqtt_table = mqtt_value["mqtt"].as_table().unwrap();

    let mqtt_config = read_config::read_mqtt(mqtt_table);
    log::info!("MQTT server {}:{}", mqtt_config.host, mqtt_config.port);

    let (client, connection) = setup_mqtt(mqtt_config);
    monitor(client, connection, &launch_data);
}
