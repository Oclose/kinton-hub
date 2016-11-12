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

extern crate hyper;

const KINTON_API_URL: &'static str = "http://api.testing.kinton.io/api";

use self::hyper::client::Client;
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
    pub fn new(device_uid: String, fleet_key: String) -> Result<Device, &'static str> {
        if DEVICES.lock().unwrap().contains_key(&device_uid) {
            return Err("Device already exists");
        }

        // Register device on Kinton
        let http_client = Client::new();
        let register_url = format!("{}/fleets/{}/registerMote", KINTON_API_URL, fleet_key);
        let mut http_response = http_client.post(&register_url).body("").send().unwrap();
        let ref mut response = String::new();
        http_response.read_to_string(response).unwrap();
        let parsed_response = json::parse(response).unwrap();

        if parsed_response["uuid"].is_null() || parsed_response["secret"].is_null() {
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
    #[test]
    #[ignore]
    fn test_already_exists_device() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_register_device() {
        unimplemented!();
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
