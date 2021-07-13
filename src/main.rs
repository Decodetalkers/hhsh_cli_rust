use cli_table::{print_stdout, Cell, Style, Table};
use futures::executor::block_on;
use regex::Regex;
use serde_json::{json, Value};
use std::env;
fn get_hhsh_str(hhsh: String) -> String {
    let re = Regex::new(r"([a-zA-z0-9]{2,})+").unwrap();
    let mut output = String::new();
    for pair in re.captures_iter(hhsh.as_str()) {
        output.push_str(&pair[0]);
        output.push(',');
    }
    output.pop();
    output
}
async fn test(input: String) -> surf::Result<Value> {
    let hhsh_guess_url = "https://lab.magiconch.com/api/nbnhhsh/guess";
    let res = surf::post(hhsh_guess_url)
        .header("content-type", "application/json")
        .body(json!({ "text": input }))
        .recv_string()
        .await
        .unwrap();
    let output: Value = serde_json::from_str(res.as_str()).unwrap();
    Ok(output)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let query: String = (&args[1]).to_string();
    let input = get_hhsh_str(query);
    let hhsh: Value;
    match block_on(test(input)) {
        Ok(output) => {
            hhsh = output;
        }
        Err(_) => {
            panic!("Error");
        }
    };
    let mut index = 0;
    let mut output = vec![];
    while hhsh[index] != Value::Null {
        let mut inindex = 0;
        while hhsh[index]["trans"][inindex] != Value::Null {
            output.push(vec![
                hhsh[index]["name"].to_string().cell(),
                hhsh[index]["trans"][inindex].to_string().cell(),
            ]);
            inindex += 1;
        }
        index += 1;
    }
    let table = output
        .table()
        .title(vec!["Fucking Words".cell(), "HHSH".cell()])
        .bold(true);
    assert!(print_stdout(table).is_ok());
}
