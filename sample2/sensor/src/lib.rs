pub trait Sensor<T> {
    fn init(&mut self);
    fn read(&self) -> T;
}

pub struct LightSensor {
    active: bool,
    latest: u32,
}
impl LightSensor {
    pub fn new() -> LightSensor {
        LightSensor{
            active: false,
            latest: 0,
        }
    }
}
impl Sensor<u32> for LightSensor {
    fn init(&mut self) {
        self.active = true;
        self.latest = 42;
    }
    fn read(&self) -> u32 {
        self.latest
    }
}

pub struct TemperatureSensor {
    active: bool,
    latest: f32,
}
impl TemperatureSensor {
    pub fn new() -> TemperatureSensor {
        TemperatureSensor{
            active: false,
            latest: 0.0,
        }
    }
}
impl Sensor<f32> for TemperatureSensor {
    fn init(&mut self) {
        self.active = true;
        self.latest = 35.5;
    }
    fn read(&self) -> f32 {
        self.latest
    }
}

pub fn print_sensor_value(sensor: impl Sensor<u32>) {
    println!("sensor latest = {}", sensor.read());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_light_sensor() {
        let mut sensor = LightSensor::new();
        sensor.init();
        assert_eq!(sensor.read(), 42);
    }
    #[test]
    fn test_temperature_sensor() {
        let mut sensor = TemperatureSensor::new();
        sensor.init();
        assert_eq!(sensor.read(), 35.5);
    }
}
