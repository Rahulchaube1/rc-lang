use std::collections::HashMap;
use std::io::{self, Write};

pub fn run(commands: Vec<String>) {
    let mut vars: HashMap<String, String> = HashMap::new();

    for cmd in commands {
        if cmd.starts_with("show ") {
            let msg = cmd[5..].trim_matches('"');
            println!("{}", msg);
        }

        else if cmd.starts_with("set ") {
            let parts: Vec<&str> = cmd[4..].split(" to ").collect();
            if parts.len() == 2 {
                vars.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
            }
        }

        else if cmd.starts_with("add ") {
            let parts: Vec<&str> = cmd[4..].split(" and ").collect();
            if parts.len() == 2 {
                let x = vars.get(parts[0].trim()).unwrap_or(&"0".to_string()).parse::<i32>().unwrap_or(0);
                let y = vars.get(parts[1].trim()).unwrap_or(&"0".to_string()).parse::<i32>().unwrap_or(0);
                println!("{}", x + y);
            }
        }

        else if cmd.starts_with("ask ") {
            let var_name = cmd[4..].trim();
            print!("Enter value for {}: ", var_name);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            vars.insert(var_name.to_string(), input.trim().to_string());
        }

        else if cmd.starts_with("if ") && cmd.contains(" then ") {
            let parts: Vec<&str> = cmd[3..].split(" then ").collect();
            if parts.len() == 2 {
                let cond = parts[0];
                let do_cmd = parts[1];
                let cond_parts: Vec<&str> = cond.split(" is ").collect();

                if cond_parts.len() == 2 {
                    let var_val = vars.get(cond_parts[0].trim()).unwrap_or(&"".to_string()).to_string();
                    if var_val == cond_parts[1].trim() {
                        run(vec![do_cmd.to_string()]);
                    }
                }
            }
        }

        else if cmd.starts_with("repeat ") && cmd.contains(" times ") {
            let parts: Vec<&str> = cmd[7..].split(" times ").collect();
            if parts.len() == 2 {
                let count = parts[0].trim().parse::<u32>().unwrap_or(0);
                let loop_cmd = parts[1].trim();
                for _ in 0..count {
                    run(vec![loop_cmd.to_string()]);
                }
            }
        }

        else {
            println!("Unknown command: {}", cmd);
        }
    }
}
