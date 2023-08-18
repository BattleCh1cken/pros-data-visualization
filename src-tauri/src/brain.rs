use btleplug::api::{bleuuid::BleUuid, Central as _, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager as BtleManager, Peripheral};
use futures::stream::StreamExt;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use tauri::{AppHandle, Manager};

#[derive(Debug)]
pub struct State(pub Mutex<BtData>);

#[derive(Debug)]
pub struct BtData {
    pub adapter: Option<Adapter>,
    pub brains: Option<Vec<Peripheral>>,
    pub connected_brain: Option<Peripheral>,
    pub connected: bool,
}

impl Default for BtData {
    fn default() -> Self {
        Self {
            adapter: None,
            brains: None,
            connected_brain: None,
            connected: false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
pub async fn setup(state: tauri::State<'_, State>) -> Result<(), String> {
    println!("running setup");
    let manager = BtleManager::new().await.unwrap(); // Do I need to handle this error?
    println!("manager constructed");

    let central = match manager.adapters().await {
        Ok(central) => central,
        Err(_) => return Err("Your computer does not support bluetooth".to_string()),
    };

    println!("{:?}", central);

    /*
        .await
        .expect("Unable to fetch adapter list.")
        .into_iter()
        .nth(0)
        .expect("Unable to find adapters.");

    println!("found adapter");

    state.0.lock().unwrap().adapter = Some(central);

    println!("{:?}", state);
    */

    Ok(())
}

#[tauri::command]
pub async fn find_brains(state: tauri::State<'_, State>) -> Result<(), Error> {
    println!("{:?}", state);

    /*
    central.start_scan(ScanFilter::default()).await.unwrap();
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    sleep(Duration::from_secs(2));

    let peripherals = central.peripherals().await.unwrap();
    */
    Ok(())
}
