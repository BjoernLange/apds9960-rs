#[cfg(not(target_os = "linux"))]
fn main() {}

#[cfg(target_os = "linux")]
fn main() {
    use apds9960::Apds9960;
    use linux_embedded_hal::I2cdev;
    use nb::block;

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Apds9960::new(dev);
    sensor.enable().unwrap();
    sensor.enable_proximity().unwrap();
    loop {
        let p = block!(sensor.read_proximity()).unwrap();
        println!("Proximity: {}", p);
    }
}
