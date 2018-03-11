#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate bson;
extern crate rmp_serde;
extern crate ron;
extern crate serde_cbor;
extern crate serde_json;
extern crate serde_pickle;

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use bincode::deserialize as bincode_de;
use bincode::serialize as bincode_ser;

use serde_cbor::from_slice as cbor_de;
use serde_cbor::to_vec as cbor_ser;

use rmp_serde::decode::from_slice as rmp_de;
use rmp_serde::encode::to_vec as rmp_ser;

use bson::from_bson as bson_de;
use bson::to_bson as bson_ser;

use serde_json::from_str as json_de;
use serde_json::to_string as json_ser;

use serde_pickle::from_slice as pickle_de;
use serde_pickle::to_vec as pickle_ser;

use ron::de::from_str as ron_de;
use ron::ser::to_string as ron_ser;

const SERDE_RUNS: usize = 100_000;
const RUNS: usize = 5;

fn duration_to_ms(dur: &Duration) -> f64 {
    dur.as_secs() as f64 * 1000.0 + dur.subsec_nanos() as f64 / 1_000_000.0
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum PubType {
    PressureSensorTemperature,
    PressureSensorPressure,
    LongDistanceSensor,
    // ...
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct PubMessage {
    pub_type: PubType,
    integral: i16,
    decimal: f32,
}

fn main() {
    let msg = PubMessage {
        pub_type: PubType::PressureSensorPressure,
        integral: 100,
        decimal: 100.4,
    };

    let mut perf_tests: BTreeMap<&'static str, Box<Fn(&PubMessage)>> =
        BTreeMap::new();

    perf_tests.insert(
        "bincode",
        Box::new(|msg: &PubMessage| {
            let encoded = bincode_ser(&msg).unwrap();

            for _ in 0..SERDE_RUNS {
                bincode_ser(&msg).ok();
            }
            for _ in 0..SERDE_RUNS {
                let _: PubMessage = bincode_de(&encoded[..]).unwrap();
            }
        }),
    );

    perf_tests.insert(
        "serde_cbor",
        Box::new(|msg: &PubMessage| {
            let encoded = cbor_ser(msg).unwrap();

            for _ in 0..SERDE_RUNS {
                cbor_ser(&msg).ok();
            }
            for _ in 0..SERDE_RUNS {
                let _: PubMessage = cbor_de(&encoded[..]).unwrap();
            }
        }),
    );

    perf_tests.insert(
        "rmp_serde",
        Box::new(|msg: &PubMessage| {
            let encoded = rmp_ser(msg).unwrap();

            for _ in 0..SERDE_RUNS {
                rmp_ser(&msg).ok();
            }
            for _ in 0..SERDE_RUNS {
                let _: PubMessage = rmp_de(&encoded[..]).unwrap();
            }
        }),
    );

    perf_tests.insert(
        "bson",
        Box::new(|msg: &PubMessage| {
            let encoded = bson_ser(msg).unwrap();

            for _ in 0..SERDE_RUNS {
                bson_ser(&msg).ok();
            }
            for _ in 0..SERDE_RUNS {
                let _: PubMessage = bson_de(encoded.clone()).unwrap();
            }
        }),
    );

    perf_tests.insert(
        "serde_json",
        Box::new(|msg: &PubMessage| {
            let encoded = json_ser(msg).unwrap();

            for _ in 0..SERDE_RUNS {
                json_ser(&msg).ok();
            }
            for _ in 0..SERDE_RUNS {
                let _: PubMessage = json_de(&encoded).unwrap();
            }
        }),
    );

    perf_tests.insert(
        "serde_pickle",
        Box::new(|msg: &PubMessage| {
            let encoded = pickle_ser(msg, true).unwrap();

            for _ in 0..SERDE_RUNS {
                pickle_ser(&msg, true).ok();
            }
            for _ in 0..SERDE_RUNS {
                let _: PubMessage = pickle_de(&encoded).unwrap();
            }
        }),
    );

    perf_tests.insert(
        "ron",
        Box::new(|msg: &PubMessage| {
            let encoded = ron_ser(msg).unwrap();

            for _ in 0..SERDE_RUNS {
                ron_ser(&msg).ok();
            }
            for _ in 0..SERDE_RUNS {
                let _: PubMessage = ron_de(&encoded).unwrap();
            }
        }),
    );

    let mut run_avg: f64;

    for (name, test) in &perf_tests {
        run_avg = 0.0;
        println!("{}", name);
        for run in 0..RUNS {
            let now = Instant::now();
            (*test)(&msg);
            let dur = duration_to_ms(&now.elapsed());

            run_avg += dur;

            println!("{}/{}: {:>11.5}ms", run + 1, RUNS, dur);
        }
        println!("avg: {:>11.5}ms", run_avg / RUNS as f64);
        println!("{}", "-".repeat(18));
    }
}
