#[cfg(test)]
mod test;

use std::{collections::HashMap, fs};

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct Chest {
    id: u64,
    label_id: u64,
    x_pos: f64,
    y_pos: f64,
}

#[derive(Deserialize, Debug)]
struct TakenChest {
    point_id: u64,
}

fn off_x_pos_to_uno_x_pos(x_pos: f64) -> f64 {
    0.00008581802238050456 * x_pos + 0.33314565158651044
}

fn off_y_pos_to_uno_y_pos(y_pos: f64) -> f64 {
    -0.00008584331336280794 * y_pos + 0.19941397307989506
}

fn main() {
    let data = read_json_file::<Vec<TakenChest>>("data.json").unwrap();

    let off_chests = read_json_file::<Vec<Chest>>("all_chests.json").unwrap();
    let uno_chests = read_uno_chests_file().unwrap();

    let off_uno_chests_map = off_chests_to_uno(&off_chests, &uno_chests).unwrap();

    let mut final_str = String::new();
    for chest in data {
        if let Some(found_chest) = off_uno_chests_map
            .iter()
            .find(|(off, _)| off.id == chest.point_id)
        {
            if let Some(uno_chest) = found_chest.1 {
                final_str.push_str(&format!(
                    "\nwindow._markAsFound('{}','o{}')",
                    uno_chest.id, uno_chest.label_id
                ));
            } else {
                println!("chest not found for id {}", chest.point_id)
            }
        }
    }

    fs::write("result.js", final_str).unwrap();
}

fn off_chests_to_uno<'a, 'b>(
    off_chests: &'a [Chest],
    uno_chests: &'b [Chest],
) -> Result<Vec<(&'a Chest, Option<&'b Chest>)>, String> {
    let mut uno_labels = HashMap::new();
    for chest in uno_chests {
        if !uno_labels.contains_key(&chest.label_id) {
            uno_labels.insert(chest.label_id, Vec::new());
        }

        uno_labels
            .get_mut(&chest.label_id)
            .ok_or(ERR.to_string())?
            .push(chest);
    }

    let chests = off_chests
        .iter()
        .map(|off_chest| {
            let found_chest = find_uno_chest_from_off(&off_chest, &uno_labels);
            (off_chest, found_chest)
        })
        .collect::<Vec<_>>();

    Ok(chests)
}

fn find_uno_chest_from_off<'a>(
    off_chest: &Chest,
    uno_labels: &HashMap<u64, Vec<&'a Chest>>,
) -> Option<&'a Chest> {
    let label_id = match off_chest.label_id {
        // Warming seelies
        148 => 18,
        // Shrine of depths
        577 | 509 | 411 | 212 | 9 | 8 => 8,
        _other => _other,
    };

    let uno_chests_for_label = uno_labels.get(&label_id)?;

    let uno_pos_x = off_x_pos_to_uno_x_pos(off_chest.x_pos);
    let uno_pos_y = off_y_pos_to_uno_y_pos(off_chest.y_pos);
    let (found, _dist) = uno_chests_for_label
        .iter()
        .map(|chest| {
            let x_dist = chest.x_pos - uno_pos_x;
            let y_dist = chest.y_pos - uno_pos_y;
            let dist = ((x_dist * x_dist) + (y_dist * y_dist)).sqrt();
            (chest, dist)
        })
        .reduce(|a, b| if a.1.abs() - b.1.abs() < 0.0 { a } else { b })?;

    Some(found)
}

fn read_json_file<V: for<'de> Deserialize<'de>>(name: &str) -> Result<V, String> {
    let file_str = fs::read_to_string("./".to_string() + name).map_err(|e| e.to_string())?;
    serde_json::from_str(&file_str).map_err(|e| e.to_string())
}

const ERR: &str = ":(";

fn read_uno_chests_file() -> Result<Vec<Chest>, String> {
    let file_str = fs::read_to_string("./all_chests_uno.json").map_err(|e| e.to_string())?;
    let value = serde_json::from_str::<Value>(&file_str).map_err(|e| e.to_string())?;

    fn value_to_chest(chest_value: &Value) -> Option<Chest> {
        match chest_value {
            Value::Array(chest_arr) => Some(Chest {
                id: chest_arr[0].as_number()?.as_u64()?,
                label_id: chest_arr[1].as_str()?[1..].parse().ok()?,
                x_pos: chest_arr[4].as_number()?.as_f64()?,
                y_pos: chest_arr[5].as_number()?.as_f64()?,
            }),
            _ => return None,
        }
    }

    match value {
        Value::Array(arr) => arr
            .iter()
            .map(|chest_value| value_to_chest(chest_value).ok_or(ERR.to_string()))
            .collect(),
        _ => return Err(ERR.to_string()),
    }
}
