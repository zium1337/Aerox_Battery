use aerox_5::Device;
use ksni::{Tray, MenuItem, menu::{StandardItem}, ToolTip, TrayService, Handle};

pub struct TrayHandler {
    handle: Handle<BatteryTray>,
}

impl TrayHandler {
    pub fn new(tray: BatteryTray) -> Self {
        let tray_service = TrayService::new(tray);
        let handle = tray_service.handle();
        tray_service.spawn();
        TrayHandler {
            handle,
        }
    }

    pub fn update(&self, device: &Device) {
        self.handle.update(|tray: &mut BatteryTray| { tray.update(device); })
    }

    pub fn set_status(&mut self, message: &str) {
        self.handle.update(|tray: &mut BatteryTray| { tray.set_status(message); })
    }

    pub fn clear_status(&mut self) {
        self.handle.update(|tray: &mut BatteryTray| { tray.clear_status(); })
    }
}

#[derive(Debug)]
pub struct BatteryTray {
    battery_level: u8,
    charging: bool,
    status_message: Option<String>,
    product_id: Option<u16>,
}

impl BatteryTray {
    pub fn new() -> Self {
        BatteryTray {
            battery_level: 0,
            charging: false,
            product_id: Some(0),
            status_message: Some("No device found".to_string()),
        }
    }

    pub fn update(&mut self, device: &Device) {
        self.battery_level = device.battery_level;
        self.charging = device.charging;
        self.product_id = Some(device.product_id);
    }

    pub fn set_status(&mut self, message: &str) {
        self.status_message = Some(message.to_string());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
}

impl Tray for BatteryTray {
    fn icon_name(&self) -> String {
        "input-mouse".into()
    }
    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
    fn tool_tip(&self) -> ToolTip {
        let description = match &self.status_message {
            Some(m) => m.clone(),
            None => {
                let mut description = format!("Battery level: {}%", self.battery_level);
                if self.charging {
                    description += "\nCharging";
                } else {
                    description += "\nNot charging";
                }
                description
            },
        };
        let summary_text = match self.product_id {
            Some(0x1838) | Some(0x183A) => "SteelSeries Aerox 3 Wireless",
            _ => "SteelSeries Aerox 5 Wireless",
        };
        ToolTip {
            title: summary_text.to_string(),
            description: description,
            icon_name: "".into(),
            icon_pixmap: Vec::new(),
        }
    }
}