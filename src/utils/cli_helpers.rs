use serde_json::Value;

pub fn prompt_selecter(options: Value) -> Result<String, Box<dyn std::error::Error>> {
    let options_map = options.as_object().ok_or("Options must be a JSON object")?;

    let options_vec: Vec<_> = options_map
        .iter()
        .filter_map(|(key, value)| {
            value
                .as_str()
                .map(|val| (val.to_string(), key.clone(), val.to_string()))
        })
        .collect();

    if options_vec.is_empty() {
        return Err("No valid options found".into());
    }

    use cliclack::select;

    let selected = select("Pick or Search for a script to run")
        .filter_mode()
        .items(&options_vec)
        .interact()?;

    Ok(selected)
}
