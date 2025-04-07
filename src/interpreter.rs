use std::collections::HashMap;
use std::io::{self, Write};

pub fn interpret(lines: Vec<String>) {
    let mut variables = HashMap::new();

    for line in lines {
        let line = line.trim();

        if line.starts_with("ask ") {
            let var_name = line[4..].trim();
            print!("Enter value for {}: ", var_name);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            variables.insert(var_name.to_string(), input.trim().to_string());
        }

        else if line.starts_with("show ") {
            let to_show = line[5..].trim();
            if to_show.starts_with('"') && to_show.ends_with('"') {
                println!("{}", &to_show[1..to_show.len() - 1]);
            } else if let Some(val) = variables.get(to_show) {
                println!("{}", val);
            } else {
                println!("Unknown show target: {}", to_show);
            }
        }

        else if line.starts_with("set ") && line.contains(" to ") {
            let parts: Vec<&str> = line[4..].splitn(2, " to ").collect();
            if parts.len() == 2 {
                let var = parts[0].trim();
                let value = parts[1].trim();
                variables.insert(var.to_string(), value.to_string());
            }
        }

        else if line.starts_with("repeat ") && line.contains(" times ") {
            if let Some(times_end) = line.find(" times ") {
                let count_str = &line[7..times_end];
                let count = count_str.trim().parse::<usize>().unwrap_or(0);
                let command = &line[times_end + 7..].trim();
                for _ in 0..count {
                    interpret(vec![command.to_string()]);
                }
            }
        }

        else if line.starts_with("if ") && line.contains(" then ") {
            if let Some(then_pos) = line.find(" then ") {
                let condition = &line[3..then_pos].trim();
                let command = &line[then_pos + 6..].trim();

                let parts: Vec<&str> = condition.split(" is ").collect();
                if parts.len() == 2 {
                    let var = parts[0].trim();
                    let value = parts[1].trim();
                    if let Some(stored) = variables.get(var) {
                        if stored == value {
                            interpret(vec![command.to_string()]);
                        }
                    }
                }
            }
        }

        else {
            println!("Unknown command: {}", line);
        }
    }
}
