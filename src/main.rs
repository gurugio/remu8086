mod add;
mod assembler;
mod common;
mod cpucontext;
mod inc;
mod jmp;
mod memory;
mod mov;
mod org;
mod parser;

use paste::paste;
use pest::Parser;
use std::collections::HashMap;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;

#[derive(Debug)]
pub struct ProgramLine {
    code: String,
    // BUGBUG: It will be 20-bit address with segment:address
    _start_address: u16,
    _machine_code: Vec<u8>,
    // If this line has a label,
    label: bool,
}

impl ProgramLine {
    fn new(code: &str) -> Self {
        ProgramLine {
            code: code.to_owned(),
            _start_address: 0,
            _machine_code: Vec::new(),
            label: false,
        }
    }
}

struct Hardware8086 {
    cpu: cpucontext::CpuContext,
    memory: memory::Memory,
    program: HashMap<usize, ProgramLine>,
}

impl Hardware8086 {
    fn new() -> Self {
        Self {
            cpu: cpucontext::CpuContext::boot(),
            memory: memory::Memory::boot(),
            program: HashMap::new(),
        }
    }

    /// return: next line number
    fn handle_instruction(&mut self, linenum: usize) -> Result<usize, String> {
        println!("Handle instruction:{}-line", linenum);
        let programline: &ProgramLine = self.program.get(&linenum).unwrap();
        // The line is not an instruction as like "mov ax, bx"
        // but also COMMENT, NEWLINE or WHITESPACE.
        // Therefore it uses Rule::program, not Rule::instruction when parsing the line
        // because Rule::instruction cannot handle COMMENT, NEWLINE and WHITESPACE.
        let line: &str = &programline.code;
        println!("line={:?}", line);
        let program = parser::AssemblyParser::parse(parser::Rule::program, line)
            .unwrap()
            .next()
            .unwrap();

        assert_eq!(parser::Rule::program, program.as_rule());
        let instruction = program.into_inner().next().unwrap();
        let mut nextline = linenum + 1;

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
            parser::Rule::inc => {
                caller_one!(inc, self.cpu, self.memory, instruction);
            }
            parser::Rule::label => {
                println!("Meet label and do nothing");
            }
            parser::Rule::jmp => {
                let operand = instruction.clone().into_inner().next().unwrap();
                assert_eq!(parser::Rule::name, operand.as_rule());
                caller_one!(jmp, self.cpu, self.memory, instruction);
                for (linenum, programline) in self.program.iter() {
                    println!("linenum {}: programline {}", linenum, programline.code);
                    // BUGBUG: start_with() is not best to compare "labelname:" and "labelname"
                    if programline.label == true && programline.code.starts_with(operand.as_str()) {
                        nextline = *linenum + 1;
                        println!("set next={}", nextline);
                    }
                }
            }
            _ => println!("NOT implemented yet:{}", &programline.code),
        }
        println!("After instruction: {:?}", self.cpu);

        println!("nextline={}", nextline);
        Ok(nextline)
    }

    fn reboot(&mut self) {
        self.cpu.reboot();
        self.memory.reboot();
    }

    /// Return CPU context in Json format
    /// "Reg": "value"
    fn program_response(&self, nextline: usize) -> serde_json::Value {
        let m = format!("{}", self.memory);
        serde_json::json!({
            "nextline": nextline,
            "AX": self.cpu.get_register16("ax").to_string(),
            "BX": self.cpu.get_register16("bx").to_string(),
            "CX": self.cpu.get_register16("cx").to_string(),
            "DX": self.cpu.get_register16("dx").to_string(),
            "SI": self.cpu.get_register16("si").to_string(),
            "DI": self.cpu.get_register16("di").to_string(),
            "BP": self.cpu.get_register16("bp").to_string(),
            "SP": self.cpu.get_register16("sp").to_string(),
            "CS": self.cpu.get_register16("cs").to_string(),
            "DS": self.cpu.get_register16("ds").to_string(),
            "ES": self.cpu.get_register16("es").to_string(),
            "SS": self.cpu.get_register16("ss").to_string(),
            "IP": self.cpu.get_register16("ip").to_string(),
            "FLAGS": self.cpu.get_register16("flags").to_string(),
            "memory": m,
        })
    }

    pub fn build_program_table(&mut self, program: &Vec<String>) {
        // Clear program table to read new program
        self.program.clear();
        for (i, instruction) in program.iter().enumerate() {
            let mut p = ProgramLine::new(instruction);
            if instruction.ends_with(":") {
                p.label = true;
            }
            self.program.insert(i, p);
        }
    }

    // TODO: fn get_memory(&self) -> serde_json::Value {}
}

struct HardwareLock {
    hardware: Mutex<Hardware8086>,
}

async fn handle_step(req_body: String, data: web::Data<HardwareLock>) -> impl Responder {
    println!("/step: Receive data={}", req_body);
    let mut hardware = data.hardware.lock().unwrap();
    let v: Value = serde_json::from_str(&req_body).unwrap();
    let linenum: usize = v["line"].as_u64().unwrap() as usize; // BUGBUG
    let nextline = hardware.handle_instruction(linenum).unwrap();
    HttpResponse::Ok().json(hardware.program_response(nextline))
    //HttpResponse::Ok()
}

async fn handle_reload(req_body: String, data: web::Data<HardwareLock>) -> impl Responder {
    println!("/reload: Receive data={}", req_body);
    let mut hardware = data.hardware.lock().unwrap();
    hardware.reboot();
    HttpResponse::Ok().json(hardware.program_response(0))
}

async fn handle_build(req_body: String, data: web::Data<HardwareLock>) -> impl Responder {
    println!("/build: Receive data={}", req_body);
    // req_body: {"code":["mov ax, 1","mov bx, 1"]}
    let mut hardware = data.hardware.lock().unwrap();
    hardware.reboot();
    let program: HashMap<String, Vec<String>> = serde_json::from_str(&req_body).unwrap();
    hardware.build_program_table(&program["code"]);
    println!("Build new program table: {:?}", hardware.program);
    HttpResponse::Ok().json(hardware.program_response(0))
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
            .route("/build", web::post().to(handle_build))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_main_run_example_file() {
        //
        // Actually this is not a readl test but
        // just an example to run the assembler with a local assembly source fle.
        // I made this to test the entire program on the terminal without web-things.
        //
        let mut hardware = Hardware8086::new();
        let program: Vec<String> = read_to_string("example.as")
            .unwrap()
            .lines()
            .map(|l| l.to_owned())
            .collect::<Vec<String>>();
        hardware.build_program_table(&program);
        for i in 0..program.len() {
            let _ = hardware.handle_instruction(i).unwrap();
        }
    }
}
