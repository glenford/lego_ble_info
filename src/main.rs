//
// note on macOS requires access to bluetooth to be enabled
//
// on RPi with Ubuntu needed to use: apt install pi-bluetooth
//
//
//
// LEGO BLE Docs can be found here:
//
//   https://lego.github.io/lego-ble-wireless-protocol-docs/index.html
//

use std::error::Error;
// use uuid::Uuid;
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;


const VERSION:Option<&str> = option_env!("CARGO_PKG_VERSION");
const LEGO_GROUP:u16 = 0x0397;

fn lego_system_type(t: u8) -> &'static str {
  match t {
    0x00 => "WeDo Hub",
    0x20 => "Duplo Train",
    0x40 => "Boost Hub",
    0x41 => "2 Port Hub",
    0x42 => "2 Port Handset",
    _ => "Unknown"
  }
}

fn lego_button_state(t: u8) -> &'static str {
  match t {
    0x00 => "Off",
    0x01 => "On",
    _ => "Unknown"
  }
}

fn lego_device_capabilities(t: u8) -> String {
  format!("{}|{}|{}|{}",
    if t & 0x01 > 0 { "Supports Central Role" } else { "" },
    if t & 0x02 > 0 { "Supports Peripheral Role" } else { ""},
    if t & 0x04 > 0 { "Supports LPF2 devices" } else { ""},
    if t & 0x08 > 0 { "Act as Remote Controller" } else { ""} )
}

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  println!("LEGO BLE Information Tool - v{}\n\n", VERSION.unwrap_or("version unknown"));

  // let LEGO_SERVICE_UUID:Uuid = Uuid::parse_str("00001623-1212-EFDE-1623-785FEABCD123").unwrap();

  let manager = Manager::new().await?;
  let central = get_central(&manager).await;
  let mut events = central.events().await?;

  println!("Listening.. ");

  central.start_scan(ScanFilter::default()).await?;

  while let Some(event) = events.next().await {
    match event {
      CentralEvent::DeviceDiscovered(_id) => {
      }
      CentralEvent::DeviceConnected(_id) => {
      }
      CentralEvent::DeviceDisconnected(_id) => {
      }
      CentralEvent::ManufacturerDataAdvertisement {
          id: _,
          manufacturer_data,
      } => {
          match manufacturer_data.get(&LEGO_GROUP) {
               Some(manu_data) => {
                          println!("Found LEGO device!");
                          println!("  Device: {:?}", lego_system_type(manu_data[1]));
                          println!("  Button: {:?}", lego_button_state(manu_data[0]));
                          println!("  Device Capabilities: {:?}", lego_device_capabilities(manu_data[2]));
                          println!("  Last ID: {:?}", manu_data[3]);
                          println!("  Status: {:?}", manu_data[4]);
                          println!("  Option: {:?}", manu_data[5]);
                        },
                        _ => (), // ignore
          }
      }
      CentralEvent::ServiceDataAdvertisement { id: _, service_data: _ } => {
      }
      CentralEvent::ServicesAdvertisement { id: _, services: _ } => {
      }
      _ => {
      }
    }
  }
  Ok(())  
}
