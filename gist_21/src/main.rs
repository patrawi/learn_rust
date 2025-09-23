use serde::{Deserialize, Serialize}; // Anything that can be convert to JSON need
use serde_json::{Value, json};
use std::fs;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
enum AnimalType {
    Cat,
    Duck,
    Dog,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AnimalData {
    id: String,
    animal_type: AnimalType,
    img_url: Url,
    description: Option<String>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let json_str = fs::read_to_string("./data.json")?;

    let animals: Vec<AnimalData> = serde_json::from_str::<Vec<AnimalData>>(&json_str)?;

    println!("Successfully parsed animals: {:#?}", animals);

    let pretty_json_string = serde_json::to_string_pretty(&animals)?;
    println!(
        "\nHere is the data serialized back to JSON:\n{}",
        pretty_json_string,
    );

    Ok(())
    // let foo_str = r#"[
    //         {"id": "foo", "type": "Cat", "img_url": "http://localhost:3000/assets/kat.png"},
    //         {"id": "bar", "type": "Duck", "img_url": "http://localhost:3000/assets/duck.png"}
    //     ]
    //     "#;
    // let foo_json: Vec<Value> = serde_json::from_str::<Vec<Value>>(foo_str).unwrap();
    // println!("1 foo_json = {:#?}", foo_json);

    // let filtered_foo_json = foo_json
    //     .iter()
    //     .filter(|v| v["id"] == "foo")
    //     .map(|v| v.to_owned())
    //     .collect::<Vec<_>>();
    // println!("2 filter_and_map_foo_json = {:#?}", filtered_foo_json);

    // let filtered_foo_json = foo_json
    //     .iter()
    //     .filter_map(|v| {
    //         if v["id"] == "foo" {
    //             Some(v.to_owned())
    //         } else {
    //             None
    //         }
    //     })
    //     .collect::<Vec<_>>();

    // println!("3 filter_map_foo_json = {:#?}", filtered_foo_json);

    // let filtered_foo_json = foo_json
    //     .iter()
    //     .filter(|v| v["id"] == "foo")
    //     .filter_map(|v| v["id"].as_str())
    //     .collect::<Vec<_>>();
    // println!("4 filtred_foo_value_json = {:#?}", filtered_foo_json);

    // let foo_struct = serde_json::from_str::<Vec<AnimalData>>(foo_str).unwrap();

    // println!("5 foo_struct = {:#?}", foo_struct);

    // let bar_value = json!({
    //     "id" : "bar",
    //     "type" : "Duck",
    //     "img_url": "http://localhost:3000/assets/duck.png"
    // });

    // let bar_struct = serde_json::from_value::<AnimalData>(bar_value).unwrap();

    // println!("6 bar_struct = {:#?}", bar_struct);
}
