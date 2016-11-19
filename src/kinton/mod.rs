// Copyright 2016 Diego Fernández Barrera <bigomby@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use hyper;
use std::io::Read;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;
use std::clone::Clone;
use json;
use mqttc::{ClientOptions, PubSub, PubOpt, Client};
use netopt::NetworkOptions;

lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Box<Device>>> = {
        Mutex::new(HashMap::new())
    };
}

#[derive(Clone)]
pub struct Device {
    kinton_uuid: String,
    kinton_secret: String,
    broker: String,
    mqtt_client: Arc<Mutex<Client>>,
    handlers: HashMap<String, Arc<Fn(Vec<u8>) + Send + Sync>>,
}

impl Device {
    pub fn new(client: Option<Box<hyper::Client>>,
               device_uid: String,
               fleet_key: String,
               api_url: String,
               broker: String)
               -> Result<Device, &'static str> {
        if DEVICES.lock().unwrap().contains_key(&device_uid) {
            return Err("Device already exists");
        }

        let http_client: Box<hyper::Client>;

        if let Some(c) = client {
            http_client = c;
        } else {
            http_client = Box::new(hyper::Client::new());
        }

        // Register device on Kinton
        let register_url = format!("{}/fleets/{}/registerMote", api_url, fleet_key);
        let mut http_response = http_client.post(&register_url).body("").send().unwrap();
        let ref mut response = String::new();
        http_response.read_to_string(response).unwrap();
        let parsed_response = json::parse(response).unwrap();

        if parsed_response["uuid"].is_null() || parsed_response["secret"].is_null() {
            println!("{:?}", parsed_response.dump());
            return Err("Invalid response");
        }

        // Connect to the broker
        let client = ClientOptions::new()
            .connect(broker.as_str(), NetworkOptions::new())
            .expect("Can't connect to server");

        let device = Box::new(Device {
            kinton_uuid: String::from(parsed_response["uuid"]
                .as_str()
                .unwrap()),
            kinton_secret: String::from(parsed_response["secret"]
                .as_str()
                .unwrap()),
            mqtt_client: Arc::new(Mutex::new(client)),
            broker: broker,
            handlers: HashMap::new(),
        });

        // Store device on the DB
        DEVICES.lock().unwrap().insert(device_uid, device.clone());

        Ok(*device)
    }

    pub fn subscribe<F>(&mut self, topic: String, handler: F)
        where F: Fn(Vec<u8>) + Send + Sync + 'static
    {
        self.mqtt_client.lock().unwrap().subscribe(topic.as_str()).unwrap();
        self.mqtt_client.lock().unwrap().await().unwrap();
        self.handlers.insert(topic, Arc::new(handler));
    }

    pub fn listen(&self) {
        let client = self.mqtt_client.clone();
        let handlers = self.handlers.clone();

        thread::spawn(move || {
            loop {
                if let Ok(result) = client.lock().unwrap().await() {
                    if let Some(message) = result {
                        let handler = handlers.get(&message.topic.path).unwrap();
                        handler((*message.payload).clone());
                    }
                }
            }
        });
    }

    pub fn publish(&self, topic: String, message: Vec<u8>) {
        let mut client = ClientOptions::new()
            .connect(self.broker.as_str(), NetworkOptions::new())
            .expect("Can't connect to server");

        client.publish(topic.as_str(), message, PubOpt::at_most_once()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Device;
    use hyper;
    use hyper::client::Client as HTTPClient;
    use mqttc::ClientOptions;
    use netopt::NetworkOptions;
    use std::sync::mpsc::channel;
    use std::sync::{Mutex, Arc};
    use std::collections::HashMap;

    const KINTON_API_URL: &'static str = "http://localhost";
    const KINTON_MQTT_HOST: &'static str = "localhost:1883";

    #[test]
    fn test_register_device_success() {
        mock_connector!(MockRedirectPolicy {
            "http://localhost" =>
                "HTTP/1.1 200 OK\r\n
                \r\n
                {\"uuid\": \"test_uuid\", \"secret\": \"test_secret\"}
                "
        });

        let mut client = HTTPClient::with_connector(MockRedirectPolicy::default());
        client.set_redirect_policy(hyper::client::RedirectPolicy::FollowAll);

        let device = Device::new(Some(Box::new(client)),
                                 String::from("uuid_1"),
                                 String::from("fleet_1"),
                                 String::from(KINTON_API_URL),
                                 String::from(KINTON_MQTT_HOST))
            .unwrap();

        assert!(device.kinton_uuid == "test_uuid");
        assert!(device.kinton_secret == "test_secret");
    }

    #[test]
    fn test_publish_subscribe() {

        // Connect to the broker
        let client = ClientOptions::new()
            .connect("localhost:1883", NetworkOptions::new())
            .expect("Can't connect to server");

        let mut device = Device {
            kinton_uuid: String::from("uuid_1"),
            kinton_secret: String::from("secret_1"),
            mqtt_client: Arc::new(Mutex::new(client)),
            broker: String::from("localhost:1883"),
            handlers: HashMap::new(),
        };

        let (tx, rx) = channel();
        let thread1_tx = Arc::new(Mutex::new(tx.clone()));
        let thread2_tx = Arc::new(Mutex::new(tx.clone()));
        device.subscribe(String::from("test1"), move |message| {
            let mut result = String::from_utf8(message).unwrap();
            result.push_str(" world");
            thread1_tx.lock().unwrap().send(result).unwrap();
        });
        device.subscribe(String::from("test2"), move |message| {
            let mut result = String::from_utf8(message).unwrap();
            result.push_str(" friend");
            thread2_tx.lock().unwrap().send(result).unwrap();
        });
        device.listen();

        device.publish(String::from("test1"), Vec::from("hello"));
        let data1 = rx.recv().unwrap();
        device.publish(String::from("test2"), Vec::from("bye"));
        let data2 = rx.recv().unwrap();

        assert!(data1 == "hello world");
        assert!(data2 == "bye friend");
    }
}
