mod device;
use device::*;

pub fn main() {
    let device = BleLedDevice::new();
}