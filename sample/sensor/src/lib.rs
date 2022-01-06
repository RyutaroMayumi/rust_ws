pub struct Sensor {
    active: bool,
    latest: u32,
}

impl Sensor {
    pub fn new() -> Sensor {
        Sensor {
            active: false,
            latest: 0,
        }
    }

    pub fn read(&self) -> u32 {
        self.latest
    }

    pub fn init(&mut self) {
        self.active = true;
        self.latest = 42;
    }
}

pub struct FusionSensor {
    pub temperature: Sensor,
    pub light: Sensor,
}

impl FusionSensor {
    pub fn new() -> FusionSensor {
        FusionSensor{
            temperature: Sensor::new(),
            light: Sensor::new(),
        }
    }
}

pub fn use_sensor(s: &Sensor) {
    println!("use latest value {} for something", s.latest);
}

pub fn update_sensor(s: &mut Sensor, val: u32) {
    s.latest = val;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let sensor = Sensor::new();
        assert_eq!(sensor.active, false);
        assert_eq!(sensor.latest, 0);
    }
    #[test]
    fn test_init() {
        let mut sensor = Sensor::new();
        sensor.init();
        assert_eq!(sensor.active, true);
        assert_eq!(sensor.latest, 42);
    }
    #[test]
    fn test_read() {
        let mut sensor = Sensor::new();
        sensor.init();
        let result = sensor.read();
        assert_eq!(result, 42);
    }
    #[test]
    fn test_fusion() {
        let sensor = FusionSensor::new();
        assert_eq!(sensor.temperature.active, false);
        assert_eq!(sensor.temperature.latest, 0);
        assert_eq!(sensor.light.active, false);
        assert_eq!(sensor.light.latest, 0);
    }
}
