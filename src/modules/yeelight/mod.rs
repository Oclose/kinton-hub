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

// use std::collections::HashMap;
use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use std::time::{Duration, Instant};
use std::thread;
// use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};

use httparse::{EMPTY_HEADER, Response};

use super::Device;

const DEFAULT_MX: u64 = 5;
const N_HEADERS: usize = 17;

// lazy_static! {
//     static ref DEVICES: Mutex<HashMap<String, YeelightBulb>> = {
//         Mutex::new(HashMap::new())
//     };
// }

#[derive(Debug, Clone)]
pub enum Model {
    White,
    Color,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Power {
    On,
    Off,
    Unknown,
}

#[derive(Debug, Clone)]
enum ColorMode {
    Color,
    Temperature,
    HSV,
    Unknown,
}

// YeelightBulb is a Yeelight device
#[derive(Debug, Clone)]
pub struct YeelightBulb {
    // ID is the ID of a Yeelight WiFi LED device, 3rd party device should use
    // this value to uniquely identified a Yeelight WiFi LED device.
    model: Model,

    // ID is the ID of a Yeelight WiFi LED device, 3rd party device should use
    // this value to uniquely identified a Yeelight WiFi LED device.
    id: String,

    // FWVersion is the LED device firmware version.
    fw_version: String,

    // Power is the current status of the device. "on" means the device is
    // currently turned on, "off" means it's turned off (not un-powered, just
    // software-managed off).
    power: Power,

    // Bright is the current brightness, it's the percentage of maximum
    // brightness. The range of this value is 1 ~ 100.
    bright: u8,

    // ColorMode is the current light mode. 1 means color mode, 2 means color
    // temperature mode, 3 means HSV mode.
    color_mode: ColorMode,

    // CT is the current color temperature value. The range of this value depends
    // on product model, refert to Yeelight product description. This field is
    // only valid if ColorMode is 2.
    ct: u16,

    // RGB is the current RGB value. The field is only valid if ColorMode is 1.
    rgb: u8,

    // Hue is the current hue value. The range of this value is 0 to 359. This
    // field is only valid if ColorMode is 3.
    hue: u16,

    // SAT is the current saturation value. The range of this value is 0 to 100.
    // The field is only valid if ColorMode is 3.
    sat: u8,

    // Name of the device. User can use “set_name” to store the name on the
    // device. The maximum length is 64 bytes. If none-ASCII character is used,
    // it is suggested to BASE64 the name first and then use “SetName” to store
    // it on device.
    name: String,
}

impl YeelightBulb {
    // Creates a new YeelightBulb object from a buffered response and tries to establish a
    // connection with the bulb.
    pub fn new(buf: Vec<u8>) -> Device {
        let bulb = YeelightBulb::parse(buf);

        // TODO Connect to the bulb

        Device::YeelightBulb(bulb)
    }

    // get_id is used to retrieve the ID of the smart LED.
    pub fn get_id(&self) -> Result<String, &'static str> {
        Ok(self.id.clone())
    }

    // get_prop is used to retrieve current property of smart LED.
    #[allow(unused_variables)]
    pub fn get_prop(&self) -> Result<Vec<String>, &'static str> {
        unimplemented!();
    }

    // set_ct_abx is used to change the color temperature of a smart LED.
    #[allow(unused_variables)]
    pub fn set_ct_abx(&self, ct_value: i8, effect: String, duration: i8) -> Option<&'static str> {
        unimplemented!();
    }

    // set_rgb is used to change the color of a smart LED.
    #[allow(unused_variables)]
    pub fn set_rgb(&self, rgb_value: i8, effect: String, duration: i32) -> Option<&'static str> {
        unimplemented!();
    }

    // set_hsv is used to change the color of a smart LED.
    #[allow(unused_variables)]
    pub fn set_hsv(&self,
                   hue: i8,
                   satint: i8,
                   effect: String,
                   duration: i32)
                   -> Option<&'static str> {
        unimplemented!();
    }

    // set_bright is used to change the brightness of a smart LED.
    #[allow(unused_variables)]
    pub fn set_bright(&self,
                      brightness: i8,
                      effect: String,
                      duration: i32)
                      -> Option<&'static str> {
        unimplemented!();
    }

    // set_power is used to switch on or off the smart LED (software managed on/off).
    #[allow(unused_variables)]
    pub fn set_power(&self, power: String, effect: String, duration: i32) -> Option<&'static str> {
        unimplemented!();
    }

    // toggle is used to toggle the smart LED.
    #[allow(unused_variables)]
    pub fn toggle(&self) -> Option<&'static str> {
        unimplemented!();
    }

    // set_default is used to save current state of smart LED in persistent memory. So if user
    // powers off and then powers on the smart LED again (hard power reset), the smart LED will
    // show last saved state.
    #[allow(unused_variables)]
    pub fn set_default(&self) -> Option<&'static str> {
        unimplemented!();
    }

    // start_cf is used to start a color flow. Color flow is a series of smart LED visible state
    // changing. It can be brightness changing, color changing or color temperature changing. This
    // is the most powerful command. All our recommended scenes, e.g. Sunrise/Sunset effect is
    // implemented using this method. With the flow expression, user can actually "program" the
    // light effect.
    #[allow(unused_variables)]
    pub fn start_cf(&self, count: i8, action: i8, flow_expression: String) -> Option<&'static str> {
        unimplemented!();
    }

    // stop_cf is used to stop a running color flow.
    #[allow(unused_variables)]
    pub fn stop_cf(&self) -> Option<&'static str> {
        unimplemented!();
    }

    // set_scene is used to set the smart LED directly to specified state. If the smart LED is off,
    // then it will turn on the smart LED firstly and then apply the specified command.
    #[allow(unused_variables)]
    pub fn set_scene(&self, class: String, val1: i8, val2: i8, val3: i8) -> Option<&'static str> {
        unimplemented!();
    }

    // cron_add is used to start a timer job on the smart LED
    #[allow(unused_variables)]
    pub fn cron_add(&self, cron_type: i8, value: i8) -> Option<&'static str> {
        unimplemented!();
    }

    // cron_get is used to retrieve the setting of the current cron job of the specified type.
    #[allow(unused_variables)]
    pub fn cron_get(&self) -> Result<(i8, i8), &'static str> {
        unimplemented!();
    }

    // cron_del is used to stop the specified cron job.
    #[allow(unused_variables)]
    pub fn cron_del(&self, cron_type: i8) -> Option<&'static str> {
        unimplemented!();
    }

    // set_adjust is used to change brightness, CT or color of a smart LED without knowing the
    // current value, it's main used by controllers.
    #[allow(unused_variables)]
    pub fn set_adjust(&self, action: String, prop: String) -> Option<&'static str> {
        unimplemented!();
    }

    // set_music is used to start or stop music mode on a device. Under music mode, no property will
    // be reported and no message quota is checked.
    #[allow(unused_variables)]
    pub fn set_music(&self, action: i8, host: String, port: i16) -> Option<&'static str> {
        unimplemented!();
    }

    // SetName is used to name the device. The name will be stored on the device and reported in
    // discovering response. User can also read the name through "GetProp" method.
    #[allow(unused_variables)]
    pub fn set_name(&self, name: String) -> Option<&'static str> {
        unimplemented!();
    }

    // Parses a buffered response from a Yeelight Bulb and creates a new device object.
    fn parse(buf: Vec<u8>) -> YeelightBulb {
        let mut headers = [EMPTY_HEADER; N_HEADERS];
        let mut data = Response::new(&mut headers);
        let mut bulb = YeelightBulb {
            model: Model::Unknown,
            id: String::from(""),
            fw_version: String::from(""),
            power: Power::Unknown,
            bright: 0,
            color_mode: ColorMode::Unknown,
            ct: 0,
            rgb: 0,
            hue: 0,
            sat: 0,
            name: String::from(""),
        };

        data.parse(&buf).unwrap();

        for header in data.headers {
            match header.name {
                "id" => bulb.id = String::from_utf8(header.value.to_vec()).unwrap(),
                "model" => {
                    let model = String::from_utf8(header.value.to_vec()).unwrap();
                    match model.as_str() {
                        "mono" => bulb.model = Model::White,
                        "color" => bulb.model = Model::Color,
                        _ => {
                            bulb.model = Model::Unknown;
                            println!("Unknown \"model\": {:?}", model);
                        }
                    }
                }
                "fw_ver" => bulb.fw_version = String::from_utf8(header.value.to_vec()).unwrap(),
                "power" => {
                    let power = String::from_utf8(header.value.to_vec()).unwrap();
                    match power.as_str() {
                        "on" => bulb.power = Power::On,
                        "off" => bulb.power = Power::Off,
                        _ => {
                            bulb.power = Power::Unknown;
                            println!("Unknown \"power\" value: {:?}", power);
                        }
                    }
                }
                "bright" => {
                    bulb.bright = String::from_utf8(header.value.to_vec())
                        .unwrap()
                        .parse::<u8>()
                        .unwrap()
                }
                "color_mode" => {
                    let color_mode = String::from_utf8(header.value.to_vec()).unwrap();
                    match color_mode.as_str() {
                        "1" => bulb.color_mode = ColorMode::Color,
                        "2" => bulb.color_mode = ColorMode::Temperature,
                        "3" => bulb.color_mode = ColorMode::HSV,
                        _ => {
                            bulb.color_mode = ColorMode::Unknown;
                            println!("Unknown \"color_mode\" value: {:?}", color_mode);
                        }
                    }
                }
                "ct" => {
                    bulb.ct = String::from_utf8(header.value.to_vec())
                        .unwrap()
                        .parse::<u16>()
                        .unwrap()
                }
                "rgb" => {
                    bulb.rgb = String::from_utf8(header.value.to_vec())
                        .unwrap()
                        .parse::<u8>()
                        .unwrap()
                }
                "hue" => {
                    bulb.hue = String::from_utf8(header.value.to_vec())
                        .unwrap()
                        .parse::<u16>()
                        .unwrap()
                }
                "sat" => {
                    bulb.sat = String::from_utf8(header.value.to_vec())
                        .unwrap()
                        .parse::<u8>()
                        .unwrap()
                }
                "name" => bulb.name = String::from_utf8(header.value.to_vec()).unwrap(),
                _ => {}
            }
        }

        bulb
    }
}

/**
 * Sends multicast SSDP M-SEARCH messages periodically to Yeelight Bulbs. When a bulb it's detected
 * sends the info throuh a channel.
 */
pub fn find_devices(interval_ms: u64) -> Receiver<Device> {
    let (tx, rx) = channel();
    let src = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);
    let dst = SocketAddrV4::new(Ipv4Addr::new(239, 255, 255, 250), 1982);

    thread::spawn(move || {
        let mut last_iteration = Instant::now() - Duration::from_millis(interval_ms);
        loop {
            if Instant::now().duration_since(last_iteration) >= Duration::from_millis(interval_ms) {
                last_iteration = Instant::now();
                let socket = UdpSocket::bind(src).expect("Can't bind recv socket");
                let timeout = Some(Duration::from_secs(DEFAULT_MX));
                socket.set_read_timeout(timeout).expect("Can't set read timeout");

                let discover_message = format!("M-SEARCH * HTTP/1.1\r\nHOST: {host}\r\nST: \
                                                wifi_bulb\r\nMAN: \"ssdp:discover\"\r\nMX: \
                                                {mx}\r\n\n\n",
                                               host = dst,
                                               mx = DEFAULT_MX);

                if let Err(err) = socket.send_to(&discover_message.into_bytes(), dst) {
                    println!("{:?}", err);
                    continue;
                }

                let mut buf: [u8; 1024] = [0; 1024];
                if let Ok(result) = socket.recv_from(&mut buf) {
                    let bulb = YeelightBulb::new(buf[0..result.0].to_vec());
                    tx.send(bulb).unwrap();
                }
            }
        }
    });

    rx
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use modules::yeelight::find_devices;
    use modules::Device;

    #[test]
    #[ignore]
    fn test_find_bulbs() {
        let devices_rx = find_devices(5000);
        let device = devices_rx.recv_timeout(Duration::from_millis(1000));

        if let Device::YeelightBulb(bulb) = device.unwrap() {
            let id = bulb.get_id().unwrap();
            println!("{:?}", id);
        }
    }
}
