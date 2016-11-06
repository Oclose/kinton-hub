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

use yeelight_bulb;

enum Power {
    On,
    Off,
}

enum ColorMode {
    Color,
    Hue,
    Sat,
}

// WhiteBulb is a Yeelight White device
pub struct WhiteBulb {
    // ID is the ID of a Yeelight WiFi LED device, 3rd party device should use
    // this value to uniquely identified a Yeelight WiFi LED device.
    ID: u8,

    // FWVersion is the LED device firmware version.
    FwVersion: String,

    // Power is the current status of the device. "on" means the device is
    // currently turned on, "off" means it's turned off (not un-powered, just
    // software-managed off).
    Power: Power,

    // Bright is the current brightness, it's the percentage of maximum
    // brightness. The range of this value is 1 ~ 100.
    Bright: u8,

    // ColorMode is the current light mode. 1 means color mode, 2 means color
    // temperature mode, 3 means HSV mode.
    ColorMode: ColorMode,

    // CT is the current color temperature value. The range of this value depends
    // on product model, refert to Yeelight product description. This field is
    // only valid if ColorMode is 2.
    CT: u16,

    // RGB is the current RGB value. The field is only valid if ColorMode is 1.
    RGB: u8,

    // Hue is the current hue value. The range of this value is 0 to 359. This
    // field is only valid if ColorMode is 3.
    HUE: u16,

    // SAT is the current saturation value. The range of this value is 0 to 100.
    // The field is only valid if ColorMode is 3.
    SAT: u8,

    // Name of the device. User can use “set_name” to store the name on the
    // device. The maximum length is 64 bytes. If none-ASCII character is used,
    // it is suggested to BASE64 the name first and then use “SetName” to store
    // it on device.
    Name: String,
}

impl YeelightBulb for WhiteBulb {}
