#[macro_use]
extern crate rouille;
extern crate mersh;
extern crate serde;
extern crate serde_json;

use std::sync::{Mutex};

fn main() {

    let interpreter = Mutex::new(mersh::interpreter::Interpreter::default());

    rouille::start_server("localhost:8000", move |request| router!(request,
       (GET) (/) => {

            let mut contents = String::from(HEAD);
            contents.push_str("\n");
            contents.push_str(STYLE);
            contents.push_str("<body>\n");
            for cmd in interpreter.lock().unwrap().cmd_history.iter() {
               contents.push_str(&serde_json::to_string(cmd).unwrap());
               contents.push_str("\n");
            }
            contents.push_str(CMD_FORM);
            contents.push_str("</body>\n</html>");

            rouille::Response::html(contents)
         },
       (POST) (/) => {

            let input = try_or_400!(post_input!(request, { cmd: String }));

            let cmd : mersh::interpreter::Cmd = serde_json::from_str(&input.cmd).unwrap();
            interpreter.lock().unwrap().apply_cmd(cmd); // => when apply_cmd() panics, mutex is poisoned !
            rouille::Response::redirect_302("/")

         },
       _ => rouille::Response::empty_404()
    ));
}

// The HTML document of the home page.
static HEAD: &'static str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <link rel="stylesheet" href="styles.css">
    <title>Mersh server</title>
</head>"#;

static STYLE: &'static str = r#"
<style>
    input {
      padding: 10px;
      font-size: 1.2em;
      width: 90%;
      font-family: sans-serif;
    }

    input::placeholder-shown {
      border-color: silver;
    }

    input::placeholder {
      color: grey;
    }
</style>
"#;

static CMD_FORM: &'static str = r#"
<form action="/" method="POST" enctype="multipart/form-data">
    <p><input id="invite_cmd" type="text" name="cmd" placeholder="Add mersh command"/></p>
</form>
"#;