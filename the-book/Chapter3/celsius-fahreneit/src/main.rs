// the conversion between the fahreneit and the celsius is linear
// and we have 2 easy meeting points: -40F=-40C, 32F=0C

// ps I do not know yet how to take an input, thus I will simply have constant values

fn main() {
    let factor_celsius_to_farheneit : f32 = (32.0 - (-40.0))/(0.0 - (-40.0));
    let shift_celsius_to_fahreneit : f32 = 32.0;

    let celsius_temperature : f32 = 37.0;

    println!("The original celsius_temperature is {}", celsius_temperature);
    println!("The converted farheneit_temperature is {}", shift_celsius_to_fahreneit + celsius_temperature * factor_celsius_to_farheneit);
}
