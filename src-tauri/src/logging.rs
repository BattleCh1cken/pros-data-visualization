use rand::random;
use rand::rngs::ThreadRng;
use rand::{distributions::Uniform, Rng};
use serde::ser::SerializeStruct;
use std::mem::size_of;
use std::thread::sleep;
use std::{error::Error, time::Duration};
use tauri::{AppHandle, Manager};

#[derive(Clone, Debug, Default, serde::Deserialize)]

pub struct ChartData(Vec<ChartDataSlice>);

impl serde::Serialize for ChartData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let p: Vec<i32> = self.0.iter().map(|data| data.p).collect();
        let i: Vec<i32> = self.0.iter().map(|data| data.i).collect();
        let d: Vec<i32> = self.0.iter().map(|data| data.d).collect();
        let time: Vec<u32> = self.0.iter().map(|data| data.time).collect();
        let mut state = serializer.serialize_struct("ChartData", 4)?;
        state.serialize_field("p", &p)?;
        state.serialize_field("i", &i)?;
        state.serialize_field("d", &d)?;
        state.serialize_field("time", &time)?;
        state.end()
    }
}

impl ChartData {
    fn flush_into(&mut self, other: &mut ChartData, limit: usize) {
        other.0.extend(self.0.clone()); // This is so memory inefficient and idgaf
        if other.0.len() > limit {
            let end = other.0.len();
            let beggining = end - limit;
            other.0 = other.0[beggining..end].to_vec();
        }
        self.0 = vec![]
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ChartDataSlice {
    p: i32,
    i: i32,
    d: i32,
    time: u32,
}

impl ChartDataSlice {
    pub fn random(rand: &mut ThreadRng, millis: u32) -> Self {
        Self {
            p: rand.gen_range(-127..127),
            i: rand.gen_range(-127..127),
            d: rand.gen_range(-127..127),
            time: millis,
        }
    }
}

pub async fn logging_loop(handle: AppHandle) -> Result<(), Box<dyn Error>> {
    let mut millis: u32 = 0;
    let mut rng = rand::thread_rng();
    let buffer_length = 20;
    let data_length = 1000;

    let mut global_chart_data = ChartData::default();
    let mut buffer_chart_data = ChartData::default();

    loop {
        let slice = ChartDataSlice::random(&mut rng, millis);
        buffer_chart_data.0.push(slice);

        if buffer_chart_data.0.len() > buffer_length {
            buffer_chart_data.flush_into(&mut global_chart_data, data_length);
            println!("{:#?}", global_chart_data.0.len());

            handle
                .emit_all("chart_data_update", global_chart_data.clone())
                .unwrap();
        }
        millis += 20;
        sleep(Duration::from_millis(20));
    }
}
