mod add;
mod common;
mod cpucontext;
mod memory;
mod mov;
mod org;
mod parser;

use paste::paste;
use pest::Parser;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;

struct Hardware8086 {
    cpu: cpucontext::CpuContext,
    memory: memory::Memory,
}

impl Hardware8086 {
    fn new() -> Self {
        Self {
            cpu: cpucontext::CpuContext::boot(),
            memory: memory::Memory::boot(),
        }
    }

    fn handle_instruction(&mut self, line: &str) -> Result<(), String> {
        // TODO: parse one instruction and change status of CPU and memory
        println!("Handle instruction:{}", line);
        let v: Value = serde_json::from_str(line).unwrap();
        let v = v["code"].as_str().unwrap();

        let instruction = parser::AssemblyParser::parse(parser::Rule::instruction, v)
            .unwrap()
            .next()
            .unwrap();

        match instruction.as_rule() {
            parser::Rule::mov => {
                caller_two!(mov, self.cpu, self.memory, instruction);
            }
            parser::Rule::org => {
                caller_one!(org, self.cpu, self.memory, instruction);
            }
            parser::Rule::add => {
                caller_two!(add, self.cpu, self.memory, instruction);
            }
            _ => println!("NOT implemented yet:{}", line),
        }
        println!("After instruction: {:?}", self.cpu);

        Ok(())
    }

    fn reboot(&mut self) {
        self.cpu.reboot();
        self.memory.reboot();
    }

    /// Return CPU context in Json format
    /// "Reg": "value"
    fn get_hardware(&self) -> serde_json::Value {
        let m = format!("{}", self.memory);
        serde_json::json!({
            "AX": self.cpu.get_register("ax").unwrap().to_string(),
            "BX": self.cpu.get_register("bx").unwrap().to_string(),
            "CX": self.cpu.get_register("cx").unwrap().to_string(),
            "DX": self.cpu.get_register("dx").unwrap().to_string(),
            "SI": self.cpu.get_register("si").unwrap().to_string(),
            "DI": self.cpu.get_register("di").unwrap().to_string(),
            "BP": self.cpu.get_register("bp").unwrap().to_string(),
            "SP": self.cpu.get_register("sp").unwrap().to_string(),
            "CS": self.cpu.get_register("cs").unwrap().to_string(),
            "DS": self.cpu.get_register("ds").unwrap().to_string(),
            "ES": self.cpu.get_register("es").unwrap().to_string(),
            "SS": self.cpu.get_register("ss").unwrap().to_string(),
            "IP": self.cpu.get_register("ip").unwrap().to_string(),
            "FLAGS": self.cpu.get_register("flags").unwrap().to_string(),
            "memory": m,
        })
    }

    // TODO: fn get_memory(&self) -> serde_json::Value {}
}

struct HardwareLock {
    hardware: Mutex<Hardware8086>,
}

async fn handle_step(req_body: String, data: web::Data<HardwareLock>) -> impl Responder {
    println!("/step: Receive data={}", req_body);
    let mut server = data.hardware.lock().unwrap();
    server.handle_instruction(&req_body).unwrap();
    HttpResponse::Ok().json(server.get_hardware())
}

async fn handle_reload(req_body: String, data: web::Data<HardwareLock>) -> impl Responder {
    println!("/reload: Receive data={}", req_body);
    let mut server = data.hardware.lock().unwrap();
    server.reboot();
    HttpResponse::Ok().json(server.get_hardware())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Rust web-server started at 127.0.0.1:8080");

    let myserverdata = web::Data::new(HardwareLock {
        hardware: Mutex::new(Hardware8086::new()),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    //.allowed_origin("http://127.0.0.1:8080") // 특정 도메인을 허용
                    .allow_any_origin() // Allow all domain: necessary for local file index.html
                    .allowed_methods(vec!["GET", "POST"]) // 허용할 HTTP 메서드
                    .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .app_data(myserverdata.clone())
            .route("/step", web::post().to(handle_step))
            .route("/reload", web::post().to(handle_reload))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_main_rune_example_file() {
        //
        // Actually this is not a readl test but
        // just an example to run the assembler with a local assembly source fle.
        // I made this to test the entire program on the terminal without web-things.
        //
        let mut cpu = cpucontext::CpuContext::boot();
        let mut memory = memory::Memory::boot();

        let unparsed_file = fs::read_to_string("example.as").expect("cannot read file");
        let file = parser::AssemblyParser::parse(parser::Rule::program, &unparsed_file)
            .expect("Failed to parse a file with Rule::program rule") // unwrap the parse result
            .next()
            .unwrap(); // get and unwrap the `file` rule; never fails
        for line in file.into_inner() {
            println!("Execute:{}", line.as_str());
            match line.as_rule() {
                parser::Rule::mov => {
                    caller_two!(mov, cpu, memory, line);
                }
                parser::Rule::org => {
                    caller_one!(org, cpu, memory, line);
                }
                parser::Rule::add => {
                    caller_two!(add, cpu, memory, line);
                }
                _ => println!("NOT implemented yet:{}", line),
            }
            println!("{:?}", cpu);
        }
    }
}
