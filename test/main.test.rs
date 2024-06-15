fn main() {
    let data = r#"
    {
        "name": "db",
        "module": "index.ts",
        "type": "module",
        "scripts": {
            "dev": "bun --watch index.ts",
            "build": "bun build index.ts",
            "start": "NODE_ENV=production bun index.ts",
            "test": "bun test"
        },
        "devDependencies": {
            "@types/bun": "latest"
        },
        "peerDependencies": {
            "typescript": "^5.0.0"
        },
        "dependencies": {
            "consola": "^3.2.3",
            "elysia": "^1.0.23",
            "surrealdb.js": "^1.0.0-beta.9"
        }
    }
    "#;

    let scripts = match package_json_handler(data) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if scripts == Value::Null {
        println!("No scripts found.");
        return;
    }

    // let keys = get_script_keys(&scripts);
    // println!("Scripts: {:?}", keys);

    let values = get_script_values(&scripts, "dev");

    println!("Scripts: {:?}", values);
}
