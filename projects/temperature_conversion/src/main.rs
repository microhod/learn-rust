use std::{
    fmt,
    io::{self, Write},
    str::FromStr,
};

// https://doc.rust-lang.org/book/ch03-05-control-flow.html
fn main() -> Result<(), String> {
    let temp = match read_temp("input a temperature to convert") {
        Ok(temp) => temp,
        Err(err) => return Err(err.to_string()),
    };

    println!("Opposite temp = {}", temp.in_units(temp.units.opposite()));
    return Ok(());
}

fn read_temp(prompt: &str) -> Result<Temperature, String> {
    print!("{prompt}: ");
    if let Err(err) = io::stdout().flush() {
        return Err(err.to_string());
    }

    let mut temp = String::new();
    if let Err(err) = io::stdin().read_line(&mut temp) {
        return Err(err.to_string());
    }

    if temp.len() < 2 {
        return Err("expected temperature of the form '10.5F' or '20C'".to_string());
    }
    let (value, units) = temp.split_at(temp.len() - 2);

    let value = match value.trim().parse::<f64>() {
        Ok(num) => num,
        Err(err) => return Err(format!("{err}: {}", value.trim())),
    };
    let units = match units.trim().parse::<Units>() {
        Ok(num) => num,
        Err(err) => return Err(format!("{err}: {}", units.trim())),
    };

    return Ok(Temperature { value, units });
}

#[derive(Copy, Clone)]
struct Temperature {
    value: f64,
    units: Units,
}

impl Temperature {
    fn in_units(self, units: Units) -> Temperature {
        if units == self.units {
            return self;
        }

        match self.units {
            Units::Celsius => {
                return Temperature {
                    value: (self.value * 9.0 / 5.0) + 32.0,
                    units: Units::Farenheit,
                }
            }
            Units::Farenheit => {
                return Temperature {
                    value: (self.value - 32.0) * 5.0 / 9.0,
                    units: Units::Celsius,
                }
            }
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:.2}{}", self.value, self.units);
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Units {
    Farenheit,
    Celsius,
}

impl Units {
    fn opposite(self) -> Units {
        match self {
            Units::Celsius => Units::Farenheit,
            Units::Farenheit => Units::Celsius,
        }
    }
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Units::Celsius => write!(f, "°C"),
            Units::Farenheit => write!(f, "°F"),
        };
    }
}

impl FromStr for Units {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "C" => return Ok(Self::Celsius),
            "F" => return Ok(Self::Farenheit),
            _ => return Err("invalid temperature unit"),
        }
    }
}
