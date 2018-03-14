#[macro_use]
extern crate rouille;
extern crate mersh;
extern crate serde;
extern crate serde_json;

use std::io::Read;

fn main() {

    rouille::start_server("localhost:8000", move |request| router!(request,
       (GET) (/) => {

            let mut file = std::fs::File::open("D:/Alexandre/Codes/mersh_server/src/form.html").expect("Can't open file.");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Can't read file");
            rouille::Response::html(contents)

         },
       (POST) (/) => {

            let input = try_or_400!(post_input!(request, { cmd: String }));

            let cmd : mersh::interpreter::Cmd = serde_json::from_str(&input.cmd).unwrap();

            let mut interpreter = mersh::interpreter::Interpreter::default();
            interpreter.apply_cmd(cmd);
            rouille::Response::text(serde_json::to_string(&interpreter).unwrap())
         },
       _ => rouille::Response::empty_404()
    ));

}