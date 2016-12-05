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
extern crate mqttc;
extern crate netopt;
extern crate httparse;

#[macro_use]
extern crate yup_hyper_mock as hyper_mock;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate json;

pub mod kinton;
pub mod modules;

fn main() {
    modules::init();
    loop {}
}
