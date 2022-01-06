use docs::double;
use sensor::Sensor;
use mytype::*;
use docs::double_with_error;
use sensor::FusionSensor;
use sensor::use_sensor;
use sensor::update_sensor;
use image::Image;
use point::Point;

fn main() {

    // usecase for docs::double
    let x = double(10);
    println!("10 x 2 = {}", x);

    // usecase for sensor::Sensor
    let mut sensor = Sensor::new();
    sensor.init();
    let latest = sensor.read();
    println!("latest = {}", latest);

    // usecase for mytype
    // TypeWithoutValue::print_type(TypeWithoutValue::Int);
    // TypeWithoutValue::print_type(TypeWithoutValue::Float);
    // TypeWithoutValue::print_type(TypeWithoutValue::Boolean);
    // TypeWithValue::print_type(TypeWithValue::Int(42));
    // TypeWithValue::print_type(TypeWithValue::Float(4.2));
    // TypeWithValue::print_type(TypeWithValue::Boolean(false));
    TypeWithoutValue::Int.print_type();
    TypeWithoutValue::Float.print_type();
    TypeWithoutValue::Boolean.print_type();
    TypeWithValue::Int(42).print_type();
    TypeWithValue::Float(4.2).print_type();
    TypeWithValue::Boolean(false).print_type();

    // usecase for docs::double_with_error
    if let Ok(i) = double_with_error(10) {
        println!("10 x 2 = {}", i);
    }
    if let Err(e) = double_with_error(u32::MAX) {
        println!("func failed with {:?}", e);
    }
    if let Err(e) = double_with_error(0) {
        println!("func failed with {:?}", e);
    }

    // usecase for sensor::FusionSensor
    let fusion_sensor = FusionSensor::new();
    let mut temperature_sensor = fusion_sensor.temperature;
    let mut light_sensor = fusion_sensor.light;
    println!("temperature sensor latest = {}", temperature_sensor.read());
    println!("light sensor latest = {}", light_sensor.read());
    // compile error
    // println!("temperature sensor latest = {}", fusion_sensor.temperature.read());
    // println!("light sensor latest = {}", fusion_sensor.light.read());

    // usecase for sensor::use_sensor
    use_sensor(&temperature_sensor);
    use_sensor(&light_sensor);

    // usecase for sensor::update_sensor
    update_sensor(&mut temperature_sensor, 42);
    println!("temperature sensor latest = {}", temperature_sensor.read());
    update_sensor(&mut light_sensor, 24);
    println!("light sensor latest = {}", light_sensor.read());

    // usecase for Image
    let image;
    {
        let bytes = [0; 256];
        image = Image {
            raw: &bytes,
        };
        println!("the first byte in Image = {}", image.raw[0]);
    }
    // compile error
    // println!("the first byte in Image = {}", image.raw[0]);

    // usecase for point::Point
    let p_i32 = Point::<i32>::new(300, 400);
    println!("(x,y) = ({},{})", p_i32.get_x(), p_i32.get_y());
    let p_i8 = Point::<i8>::new(10, 20);
    println!("(x,y) = ({},{})", p_i8.get_x(), p_i8.get_y());
}
