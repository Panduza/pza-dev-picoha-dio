/// Key for the usb serial in the json settings
static USB_SERIAL_KEY: &str = "usb_serial";

/// Usb settings for devices
pub struct Settings {
    /// VID
    pub vendor: Option<u16>,

    /// PID
    pub model: Option<u16>,

    /// Serial String
    pub serial: Option<String>,
}

impl Settings {

    /// Creates a new Settings instance
    /// 
    pub fn new() -> Settings {
        Settings {
            vendor: None,
            model: None,
            serial: None
        }
    }

    /// Set the vendor
    /// 
    pub fn set_vendor(mut self, vendor: u16) -> Self {
        self.vendor = Some(vendor);
        self
    }

    /// Set the model
    /// 
    pub fn set_model(mut self, model: u16) -> Self {
        self.model = Some(model);
        self
    }

}


impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let vendor = self.vendor.unwrap_or(0);
        let model = self.model.unwrap_or(0);
        write!(f, "Settings {{ vendor: {:#02x}, model: {:#02x}, serial: {:?} }}",
            vendor, model, self.serial)
    }
}
