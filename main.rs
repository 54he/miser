/*
 *                        _oo0oo_
 *                       o8888888o
 *                       88" . "88
 *                       (| -_- |)
 *                       0\  =  /0
 *                     ___/`---'\___
 *                   .' \\|     |// '.
 *                  / \\|||  :  |||// \
 *                 / _||||| -:- |||||- \
 *                |   | \\\  - /// |   |
 *                | \_|  ''\---/''  |_/ |
 *                \  .-\__  '-'  ___/-. /
 *              ___'. .'  /--.--\  `. .'___
 *           ."" '<  `.___\_<|>_/___.' >' "".
 *          | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *          \  \ `_.   \_ __\ /__ _/   .-` /  /
 *      =====`-.____`.___ \_____/___.-`___.-'=====
 *                        `=---='
 *
 *   	                   * * *
 *                         | | |
 *      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 *            赛博佛祖保佑       永不宕机     永无BUG
 */
#![feature(panic_payload_as_str)]
use {
    chrono::Local,
    lazy_static::lazy_static,
    std::{
        fs::{self, OpenOptions,},
        io::Write,
    },
    tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
    },
};
/*#[derive(Deserialize, Debug)]
struct Config{
    IP:String,
    PORT:String,
    ROOT_DIR:String,
    LOG_FILE:String,
}
static CONFIG:Config{
 IP:""
PORT:
ROOT_DIE:
LOG_FILE:
}*/
static IP: &str = "0.0.0.0";
static PORT: &str = "25565";
static OK: &str = "HTTP/1.1 200 OK";
static NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
static ROOT_DIR: &str = "/server";
static LOG_FILE: &str = "/var/log/minser/connect.log";
#[tokio::main]
async fn main() {
    loop {
        tokio::spawn(async move {
            main_process().await;
        })
        .await
        .unwrap();
    }
}
async fn main_process() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload_as_str() {
            err_log(s);
        } else {
            err_log("can not log panic");
        }
    }));
    info_log(format!("Server start"));
    let listener = TcpListener::bind(format!("{IP}:{PORT}"))
        .await
        .expect("1 :{IP}:{PORT}");
    info_log(format!("Listen {IP}:{PORT}"));
    loop {
        let (socket, client) = listener
            .accept()
            .await
            .expect("3 :can't accept from listen");
        tokio::spawn(async move {
            process(socket, client).await;
        });
    }
}
async fn process(socket: tokio::net::TcpStream, client: std::net::SocketAddr) {
    info_log(format!("connect from {}", client));
    handle_client(socket).await;
    info_log(format!("connect close from {}", client));
}
async fn handle_client(mut socket: TcpStream) {
    let mut buf = [0; 1024];
    loop {
        let _n = match socket.read(&mut buf).await {
            // socket closed
            Ok(n) if n == 0 => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
        };
        let client_read = String::from_utf8_lossy(&buf).to_string();
        let get = format!("{ROOT_DIR}{}", (read_response(client_read.clone())));
        let get = &read_realget(get);
        let file = match fs::metadata(get) {
            Ok(n) => {
                if n.is_dir() {
                    format!("{get}index.html")
                } else {
                    get.clone()
                }
            }
            Err(_) => {
                wran_log(format!("2 :not found {}", get));
                format!("{ROOT_DIR}/404.html")
            }
        };
        socket
            .write_all(&response(&file).await)
            .await
            .expect("3 :can't write response");
    }
    //info_log(format!("response complete in {:?}",Instant::now() - time_begin));
}
async fn response(file: &str) -> Vec<u8> {
    //info_log(file);
    let contents: Vec<u8> = match fs::read(file) {
        Ok(n) => n,
        Err(_) => panic!("2_UE :file can't read {file}"),
    };
    let status_line = if contents == fs::read(format!("{ROOT_DIR}/404.html")).unwrap().to_vec() {
        NOT_FOUND
    } else {
        OK
    };
    let response_vec = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n",
        contents.len()
    )
    .as_bytes()
    .to_vec();
    merge_vec_u8(response_vec, contents)
}
fn merge_vec_u8(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    a.into_iter().chain(b.into_iter()).collect()
}
//read_response函数能正常运行就别碰
fn read_response(response: String) -> String {
    ((((&(((response.split_once('\n').unwrap()).0).to_string())[4..]).to_string())
        .split_once(' ')
        .unwrap())
    .0)
        .to_string()
}
fn read_realget(response: String) -> String {
    if let Some(index) = response.find('?') {
        (&response[..index]).to_string()
    } else {
        response
    }
}

fn err_log<T: std::fmt::Display>(message: T) {
    write_log(message, "ERR!!", true).unwrap();
}
fn wran_log<T: std::fmt::Display>(message: T) {
    write_log(message, "WRAN", false).unwrap();
}
fn info_log<T: std::fmt::Display>(message: T) {
    write_log(message, "info", false).unwrap();
}
fn write_log<T: std::fmt::Display>(
    message: T,
    mode_info: &str,
    mode: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let write_temp =format!(
            "{mode_info}[{}]:{}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            message
        );
    
    if mode {
        eprintln!("{}",write_temp)
    } else {
        println!("{}" ,write_temp)
    }
    write!(OpenOptions::new().append(true).open(LOG_FILE)?, "{}", mode)?;
    Ok(())
}
