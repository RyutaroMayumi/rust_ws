use sensor::Sensor; // init() and read() methods are not found error occur, if no this sentence.
use sensor::LightSensor;
use sensor::TemperatureSensor;
use sensor::print_sensor_value;

fn main() {
    let mut light_sensor = LightSensor::new();
    let mut temperature_sensor = TemperatureSensor::new();

    println!("light sensor latest = {}", light_sensor.read());
    println!("temperature sensor latest = {}", temperature_sensor.read());

    light_sensor.init();
    temperature_sensor.init();

    println!("light sensor latest = {}", light_sensor.read());
    println!("temperature sensor latest = {}", temperature_sensor.read());

    print_sensor_value(light_sensor);
    // compile error!
    // print_sensor_value(temperature_sensor);
}
