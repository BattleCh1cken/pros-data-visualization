use btleplug::api::{CharPropFlags, Manager as _, Peripheral as _};
use futures::stream::StreamExt;
use rand::random;
use std::collections::VecDeque;
use std::error::Error;
use std::{time::Duration, vec};
use tauri::{AppHandle, Manager, State};

use crate::brain;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ChartData(pub Vec<Vec<f32>>);

impl ChartData {
    pub fn push(&mut self, input: Vec<f32>) {
        if self.0.len() < input.len() {
            let length_diff = input.len() - self.0.len();
            for _ in 0..length_diff {
                self.0.push(vec![])
            }
        }
        for (index, number) in input.iter().enumerate() {
            self.0[index].push(*number);
        }
    }

    pub fn flush_into(&mut self, other: &mut ChartData, limit: usize) {
        if self.0.len() > other.0.len() {
            let length_diff = self.0.len() - other.0.len();
            for _ in 0..length_diff {
                other.0.push(vec![])
            }
        }

        for (index, vec) in self.0.iter().enumerate() {
            other.0[index].extend(vec);
            if other.0[index].len() > limit {
                let end = other.0[index].len();
                let beggining = end - limit;
                other.0[index] = other.0[index][beggining..end].to_vec()
            }
        }

        self.0 = vec![];
    }
}

pub fn decode(data: Vec<u8>) -> Result<Vec<f32>, Box<dyn Error>> {
    let mut buffer = String::new();

    for byte in data {
        buffer.push(byte as char);
    }
    buffer = buffer.replace("\n", "");

    let data: Result<Vec<f32>, Box<dyn Error>> = buffer
        .split(",")
        .map(|s| {
            s.trim()
                .parse::<f32>()
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
        .collect();

    data
}

pub async fn logging_loop(handle: AppHandle) {
    let mut chart_data = ChartData::default();
    let mut chart_data_buffer = ChartData::default();
    let state: State<'_, brain::BtState> = handle.state();
    let buffer_size = 20;
    let data_size = 100;
    loop {
        /*
        let slice: [f32; 3] = random();
        chart_data_buffer.push(slice.to_vec());
        if chart_data_buffer.0[0].len() >= buffer_size {
            chart_data_buffer.flush_into(&mut chart_data, data_size);
            handle
                .emit_all("chart_data_update", chart_data.clone())
                .unwrap();
        }
        std::thread::sleep(Duration::from_millis(20));
        */
        if state.0.lock().unwrap().connected {
            let brain = state.0.lock().unwrap().connected_brain.clone().unwrap();

            for characteristic in brain.characteristics() {
                if characteristic.uuid == brain::CHAR2_UUID
                    && characteristic.properties.contains(CharPropFlags::NOTIFY)
                {
                    println!("Subscribing to characteristic {:?}", characteristic.uuid);

                    brain.subscribe(&characteristic).await.unwrap();

                    let mut notification_stream =
                        brain.notifications().await.expect("notification L");

                    while let Some(data) = notification_stream.next().await {
                        match decode(data.value) {
                            Ok(data) => {
                                chart_data_buffer.push(data);
                                if chart_data_buffer.0[0].len() > buffer_size {
                                    println!("flushing buffer");
                                    chart_data_buffer.flush_into(&mut chart_data, data_size);
                                    handle
                                        .emit_all("chart_data_update", chart_data.clone())
                                        .unwrap();
                                    println!("Chart data:{:#?}", chart_data);
                                }
                            }
                            Err(_) => println!("error >:("),
                        };
                    }
                }
            }
            handle
                .emit_all("chart_data_update", chart_data.clone())
                .unwrap();
        } else {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
