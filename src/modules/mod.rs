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

mod yeelight;

use std::thread;

const UPDATE_INTERVAL_MS: u64 = 10 * 1000;

pub enum Device {
    YeelightBulb(yeelight::YeelightBulb),
    None,
}

pub fn init() {
    let devices_rx = yeelight::find_devices(UPDATE_INTERVAL_MS);
    thread::spawn(move || {
        loop {
            let device = devices_rx.recv().unwrap();

            match device {
                Device::YeelightBulb(bulb) => {
                    println!("{:?}", bulb);
                }
                Device::None => {}
            }
        }
    });
}
