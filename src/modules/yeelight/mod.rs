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

mod white_bulb;

use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use std::io::Error;
use self::white_bulb::WhiteBulb;

use super::Device;

pub trait YeelightBulb {
    // get_id is used to retrieve the ID of the smart LED.
    fn get_id(&self) -> Result<String, &'static str>;

    // get_prop is used to retrieve current property of smart LED.
    fn get_prop(&self) -> Result<Vec<String>, &'static str>;

    // set_ct_abx is used to change the color temperature of a smart LED.
    fn set_ct_abx(&self, ctValue: i8, effect: String, duration: i8) -> Option<&'static str>;

    // set_rgb is used to change the color of a smart LED.
    fn set_rgb(&self, rgbValue: i8, effect: String, duration: i32) -> Option<&'static str>;

    // set_hsv is used to change the color of a smart LED.
    fn set_hsv(&self, hue: i8, satint: i8, effect: String, duration: i32) -> Option<&'static str>;

    // set_bright is used to change the brightness of a smart LED.
    fn set_bright(&self, brightness: i8, effect: String, duration: i32) -> Option<&'static str>;

    // set_power is used to switch on or off the smart LED (software managed on/off).
    fn set_power(&self, power: String, effect: String, duration: i32) -> Option<&'static str>;

    // toggle is used to toggle the smart LED.
    fn toggle(&self) -> Option<&'static str>;

    // set_default is used to save current state of smart LED in persistent memory. So if user
    // powers off and then powers on the smart LED again (hard power reset), the smart LED will
    // show last saved state.
    fn set_default(&self) -> Option<&'static str>;

    // start_cf is used to start a color flow. Color flow is a series of smart LED visible state
    // changing. It can be brightness changing, color changing or color temperature changing. This
    // is the most powerful command. All our recommended scenes, e.g. Sunrise/Sunset effect is
    // implemented using this method. With the flow expression, user can actually "program" the
    // light effect.
    fn start_cf(&self, count: i8, action: i8, flowExpression: String) -> Option<&'static str>;

    // stop_cf is used to stop a running color flow.
    fn stop_cf(&self) -> Option<&'static str>;

    // set_scene is used to set the smart LED directly to specified state. If the smart LED is off,
    // then it will turn on the smart LED firstly and then apply the specified command.
    fn set_scene(&self, class: String, val1: i8, val2: i8, val3: i8) -> Option<&'static str>;

    // cron_add is used to start a timer job on the smart LED
    fn cron_add(&self, cronType: i8, value: i8) -> Option<&'static str>;

    // cron_get is used to retrieve the setting of the current cron job of the specified type.
    fn cron_get(&self) -> Result<(i8, i8), &'static str>;

    // cron_del is used to stop the specified cron job.
    fn cron_del(&self, cronType: i8) -> Option<&'static str>;

    // set_adjust is used to change brightness, CT or color of a smart LED without knowing the
    // current value, it's main used by controllers.
    fn set_adjust(&self, action: String, prop: String) -> Option<&'static str>;

    // set_music is used to start or stop music mode on a device. Under music mode, no property will
    // be reported and no message quota is checked.
    fn set_music(&self, action: i8, host: String, port: i16) -> Option<&'static str>;

    // SetName is used to name the device. The name will be stored on the device and reported in
    // discovering response. User can also read the name through "GetProp" method.
    fn set_name(&self, name: String) -> Option<&'static str>;
}

/**
 * Sends a multicast SSDP M-SEARCH message and waits responses from Yeelight Bulbs. After a
 * timeout returns a Vec with the found bulbs.
 */
fn find_devices(timeout: Option<Duration>) -> Result<Vec<Device>, Error> {
    let mut devices = Vec::new();

    let src = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);
    let dst = SocketAddrV4::new(Ipv4Addr::new(239, 255, 255, 250), 1982);
    let socket = UdpSocket::bind(src).expect("Can't bind recv socket");
    try!(socket.set_read_timeout(timeout));

    let mut mx = 0;
    if let Some(time) = timeout {
        mx = time.as_secs();
    }
    let discover_message = format!("M-SEARCH * HTTP/1.1\r\nHOST: {host}\r\nST: wifi_bulb\r\nMAN: \
                                    \"ssdp:discover\"\r\nMX: {mx}\r\n\n\n",
                                   host = dst,
                                   mx = mx);
    try!(socket.send_to(&discover_message.into_bytes(), dst));

    // TODO process more messages coming from more devices
    let mut buf: [u8; 1024] = [0; 1024];
    if let Ok(result) = socket.recv_from(&mut buf) {
        devices.push(WhiteBulb::new(buf[0..result.0].to_vec()));
    }

    Ok(devices)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use modules::yeelight::find_devices;
    use modules::Device;

    #[test]
    #[ignore]
    fn test_find_bulbs() {
        let devices = find_devices(Some(Duration::from_secs(3))).unwrap();
        for device in devices {
            if let Device::YeelightBulb(bulb) = device {
                println!("Found bulb ID: {}", bulb.get_id().unwrap());
            }
        }
    }
}
