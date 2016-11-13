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

const KINTON_API_URL: &'static str = "http://api.testing.kinton.io/api";

use hyper;
use std::io::Read;
use std::collections::HashMap;
use std::sync::Mutex;
use json;

lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Device>> = {
        Mutex::new(HashMap::new())
    };
}

#[derive(Clone)]
pub struct Device {
    kinton_uuid: String,
    kinton_secret: String,
}

impl Device {
    pub fn new(client: Option<Box<hyper::Client>>,
               device_uid: String,
               fleet_key: String)
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
        let register_url = format!("{}/fleets/{}/registerMote", KINTON_API_URL, fleet_key);
        let mut http_response = http_client.post(&register_url).body("").send().unwrap();
        let ref mut response = String::new();
        http_response.read_to_string(response).unwrap();
        let parsed_response = json::parse(response).unwrap();

        if parsed_response["uuid"].is_null() || parsed_response["secret"].is_null() {
            println!("{:?}", parsed_response.dump());
            return Err("Invalid response");
        }

        let device = Device {
            kinton_uuid: String::from(parsed_response["uuid"]
                .as_str()
                .unwrap()),
            kinton_secret: String::from(parsed_response["secret"]
                .as_str()
                .unwrap()),
        };

        // Store device on the DB
        DEVICES.lock().unwrap().insert(device_uid, device.clone());

        Ok(device)
    }

    pub fn subscribe(&self, topic: String) {
        unimplemented!();
    }

    pub fn publish(&self, topic: String, message: Vec<u8>) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::Device;
    use hyper;
    use hyper::server::{Server, Request, Response};
    use hyper::client::Client;
    use std::sync::mpsc::channel;
    use std::sync::Mutex;
    use std::io::Read;
    use std::thread;

    #[test]
    fn test_register_device_success() {
        mock_connector!(MockRedirectPolicy {
            "http://api.testing.kinton.io" =>
                "HTTP/1.1 200 OK\r\n
                \r\n
                {\"uuid\": \"test_uuid\", \"secret\": \"test_secret\"}
                "
        });

        let mut client = Client::with_connector(MockRedirectPolicy::default());
        client.set_redirect_policy(hyper::client::RedirectPolicy::FollowAll);

        let device = Device::new(Some(Box::new(client)),
                                 String::from("uuid_1"),
                                 String::from("fleet_1"))
            .unwrap();

        assert!(device.kinton_uuid == "test_uuid");
        assert!(device.kinton_secret == "test_secret");
    }

    #[test]
    #[ignore]
    fn test_subscribe() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_publish() {
        unimplemented!();
    }
}
