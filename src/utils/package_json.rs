use serde_json::{Result, Value};

// ! This function will parse the package.json file and return the scripts object.
pub fn package_json_handler(data: &str) -> Result<Value> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;
    // Access the scripts object
    if let Some(scripts) = v.get("scripts") {
        return Ok(scripts.clone());
    }
    Ok(Value::Null)
}

// ! This function will return the keys of the scripts object.
pub fn get_script_keys(scripts: &Value) -> Vec<String> {
    let mut keys = Vec::new();

    if let Value::Object(obj) = scripts {
        for (key, _) in obj.iter() {
            keys.push(key.to_string());
        }
    }
    keys
}

// ! This function will return the values of the scripts object when provided with the key.
pub fn get_script_values(scripts: &Value, key: &str) -> String {
    let mut value = String::new();

    if let Value::Object(obj) = scripts {
        if let Some(script) = obj.get(key) {
            if let Value::String(val) = script {
                value = val.to_string();
            }
        }
    }
    value
}
