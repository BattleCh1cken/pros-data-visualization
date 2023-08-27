use btleplug::api::{
    Central, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager as BtleManager, Peripheral};
use futures::stream::StreamExt;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use std::vec;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

pub const SERVICE_UUID: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb13d5);

pub const CHAR1_UUID: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb1306); // Write
pub const CHAR2_UUID: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb1316); // Read
pub const CHAR3_UUID: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb13e5); // System

#[derive(Debug)]
pub struct BtState(pub Mutex<BtStateData>);

#[derive(Debug)]
pub struct BtStateData {
    pub brains: Vec<Peripheral>,
    pub connected_brain: Option<Peripheral>,
    pub connected: bool,
}

impl Default for BtStateData {
    fn default() -> Self {
        Self {
            brains: vec![],
            connected_brain: None,
            connected: false,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrainData {
    name: String,
}

impl BrainData {
    async fn from(perf: Peripheral) -> Self {
        let name = perf
            .properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .unwrap()
            .trim()
            .to_string();

        BrainData { name }
    }
}

pub async fn find_brain_loop(handle: AppHandle) -> Result<(), String> {
    sleep(Duration::from_secs(5));
    let manager = BtleManager::new().await.unwrap(); // Do I need to handle this error?
    println!("manager constructed");

    let adapters = match manager.adapters().await {
        Ok(central) => central,
        Err(_) => return Err("Your computer does not support bluetooth".to_string()), // TODO: emit an error event here
    };

    let central = adapters.into_iter().nth(0).unwrap();

    let mut events = central.events().await.unwrap();

    let brain_filter = ScanFilter {
        // FIXME: this filter does not appear to work
        services: vec![SERVICE_UUID],
    };

    central.start_scan(brain_filter).await.unwrap();

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let brains = central.peripherals().await.unwrap();
                let mut brain_data: Vec<BrainData> = vec![];
                for brain in brains.iter() {
                    let data = BrainData::from(brain.clone()).await;
                    brain_data.push(data);
                }
                println!("{:#?}", brain_data);
                handle.emit_all("found_brain", brain_data).unwrap();

                handle.state::<BtState>().0.lock().unwrap().brains = brains;
            }
            _ => (),
        }
    }

    Ok(())
}

fn find_characteristic(brain: &Peripheral, uuid: Uuid) -> Option<Characteristic> {
    for characteristic in brain.characteristics() {
        if characteristic.uuid == uuid {
            return Some(characteristic);
        }
    }
    None
}

#[tauri::command]
pub async fn connect(index: usize, state: tauri::State<'_, BtState>) -> Result<(), String> {
    println!("{index}");
    let brain: &Peripheral = &state.0.lock().unwrap().brains.clone()[index];

    brain.connect().await.unwrap();
    brain.discover_services().await.unwrap();

    let characteristic = find_characteristic(&brain, CHAR3_UUID).unwrap();

    brain
        .write(
            &characteristic,
            &[0xff, 0xff, 0xff, 0xff], // This tells the brain to display the verification code on it's screen
            WriteType::WithResponse,
        )
        .await
        .unwrap();

    state.0.lock().unwrap().connected_brain = Some(brain.clone());

    Ok(())
}

fn convert_code(code: String) -> Vec<u8> {
    let code: Vec<u8> = code
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    code
}

#[tauri::command]
pub async fn authenticate(code: String, state: tauri::State<'_, BtState>) -> Result<(), String> {
    let brain = state.0.lock().unwrap().connected_brain.clone().unwrap();

    let code = convert_code(code);

    println!("input: {:?}", code);

    //b'\x02\x03\x08\x01'
    let characteristic = find_characteristic(&brain, CHAR3_UUID).unwrap();
    brain
        .write(&characteristic, &code, WriteType::WithResponse)
        .await
        .unwrap();

    let response = brain
        .read(&find_characteristic(&brain, CHAR3_UUID).unwrap())
        .await
        .unwrap();
    println!("echo: {:?}", response);
    if response == code {
        println!("connected");
        state.0.lock().unwrap().connected = true;
    } else {
        println!("wrong code you nerd");
    }

    Ok(())
}

#[tauri::command]
pub async fn disconnect(state: tauri::State<'_, BtState>) -> Result<(), String> {
    Ok(())
}
