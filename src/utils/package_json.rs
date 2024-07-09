use serde_json::{Result, Value};

pub fn package_json_handler(data: &str) -> Value {
    // Parse the string of data into serde_json::Value.
    serde_json::from_str(data).unwrap()
}

// ! This function will parse the package.json file and return the scripts object.
pub fn package_json_scripts_handler(data: &str) -> Result<Value> {
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

pub fn get_package_manager(package_json: &Value) -> String {
    if let Some(pkg_manager) = package_json.get("packageManager") {
        if let Some(pkg_manager_str) = pkg_manager.as_str() {
            let lowercase_pkg_manager = pkg_manager_str.to_lowercase();
            if lowercase_pkg_manager.starts_with("pnpm") {
                return "pnpm".to_string();
            } else if lowercase_pkg_manager.starts_with("yarn") {
                return "yarn".to_string();
            } else if lowercase_pkg_manager.starts_with("bun") {
                return "bun".to_string();
            } else if lowercase_pkg_manager.starts_with("npm") {
                return "npm".to_string();
            }
        }
    }

    // Default to "npm" if no packageManager is specified or if it's not recognized
    "npm run".to_string()
}
