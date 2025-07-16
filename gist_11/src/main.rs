// Rust Coding Challenge: Vehicle Management System

use std::fmt; // Import for Display trait
use std::str::FromStr;
use strum::ParseError;

#[derive(Debug, PartialEq)]
pub enum VehicleType {
    Car { make: String, model: String },
    Motorcycle,
    Bicycle,
    Boat,
    Unknown,
    Airplane,
    CustomVehicle(String),
}

impl FromStr for VehicleType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("custom:") {
            let extracted_string = &s[7..]; // We need reference here because it can't know the size of string at run time

            if extracted_string.is_empty() {
                return Err(ParseError::VariantNotFound);
            }
            Ok(VehicleType::CustomVehicle(extracted_string.to_owned()))
        } else {
            match s {
                "car" => Ok(VehicleType::Car {
                    make: "Default".to_string(),
                    model: "Default".to_string(),
                }),
                "motorcycle" => Ok(VehicleType::Motorcycle),
                "bicycle" => Ok(VehicleType::Bicycle),
                "boat" => Ok(VehicleType::Boat),
                "unknown" => Ok(VehicleType::Unknown),
                _ => Err(ParseError::VariantNotFound),
            }
        }
    }
}

impl fmt::Display for VehicleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VehicleType::Car { make, model } => write!(f, "Automobile ({}/{})", make, model),
            VehicleType::Motorcycle => write!(f, "Two Wheeler"),
            VehicleType::Bicycle => write!(f, "PedalPower"),
            VehicleType::Boat => write!(f, "Watercraft"),
            VehicleType::Unknown => write!(f, "Unknown Vehicle"),
            VehicleType::Airplane => write!(f, "Airplane"),
            VehicleType::CustomVehicle(name) => write!(f, "{}", name),
        }
    }
}
fn process_vehicle_string(input: &str) -> String {
    match VehicleType::from_str(input) {
        Ok(vehicle) => vehicle.to_string(),
        Err(err) => format!(
            "Error: Could not parse '{}' into a VehicleType. Details: {:?}",
            input, err
        ),
    }
}

fn main() {
    let vehicle_strings: Vec<&str> = vec![
        "car",
        "motorcycle",
        "bicycle",
        "boat",
        "airplane",
        "custom:Spaceship",
        "custom:Submarine",
        "train",
        "hovercraft",
    ];
    for vehicle_str in vehicle_strings.iter() {
        let result = process_vehicle_string(vehicle_str);
        println!("Processing \"{}\" : {}", vehicle_str, result);
    }

    let my_fleet: Vec<VehicleType> = vec![
        VehicleType::Car {
            make: "Toyota".to_owned(),
            model: "Camry".to_owned(),
        },
        VehicleType::Motorcycle,
        VehicleType::Airplane,
        VehicleType::Bicycle,
        VehicleType::Boat,
        VehicleType::CustomVehicle("Submarine".to_string()),
    ];
    let new_fleet = my_fleet
        .into_iter()
        .filter_map(|e| match e {
            VehicleType::CustomVehicle(name) => Some(name),
            _ => None,
        })
        .collect::<Vec<String>>();

    println!("Custom vehicles in fleet: {:?}", new_fleet.join(","));
}
