#![feature(panic_payload_as_str)]
use {
    chrono::Local,
    std::{
        fs::{self, OpenOptions},
        io::Write,
        ops::Deref,
        sync::LazyLock,
    },
    tokio::{
        io::{AsyncReadExt, AsyncWriteExt, BufReader},
        net::{TcpListener, TcpStream},
        sync::Mutex,
    },
};
type LOG_FILE_OPEN<'a> = Option<std::fs::File>;
static mut FILE_RESULT: OnceLock<LOG_FILE_OPEN> = OnceLock::new();
fn LOG_FILE_OPENER() -> RefCell<&'static LOG_FILE_OPEN> {
    RefCell::new(FILE_RESULT.get_or_init(|| {
        system_log(format!("try to open the LOG file in{LOG_FILE}...."));
        let mut file_result = OpenOptions::new().append(true).open(LOG_FILE);
        if let Ok(mut n) = file_result {
            info_log("The log file is opened successfully");
            Some(n)
        } else if let Err(e) = file_result {
            wran_log(format!(
                "==!!THE LOG FILE SYSTEM IS ERR CAUSE:{e}
              \n    THAT MEANS LOGS CANNOT BE RECORDED!
              \n    THIS WARNING WILL ONLY BE DISPLAYED ONCE!
              \n!!=="
            ));
            None
        } else {
            panic!("Undefined operation, unable to recover.")
        }
    }))
}

/*static mut LOG_FILE_OPENER: Mutex<LazyLock<Option<fs::File>>>=Mutex::new(LazyLock::new(||{
info_log("try to open the LOG file in{LOG_FILE}....");
let file_result =OpenOptions::new().append(true).open(LOG_FILE);
if let Ok(n) = file_result{
  info_log("The log file is opened successfully");
  Some(n)
}else if let Err(e) = file_result{
  wran_log(format!(
         "==!!THE LOG FILE SYSTEM IS ERR CAUSE:{e}
         \n    THAT MEANS */
const IP: &str = "0.0.0.0";
const PORT: &str = "25565";
//constHTTP_VERSION: &str = "HTTP/1.1 ";
const OK: &str = "HTTP/1 200 OK";
const NOT_FOUND: &str = "HTTP/1 404 Not Found";
const MOVED_PERMANENTLY: &str = "HTTP/1 302 Found";
const ROOT_DIR: &str = "/server";
const LOG_FILE: &str = "/var/log/minser/connect.log";
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
            err_log(format!("PANIC FOR {s}"));
        } else {
            err_log("can not log panic");
        }
    }));
    info_log(format!("Server start"));
    let listener = TcpListener::bind(format!("{IP}:{PORT}"))
        .await
        .expect("1 :{IP}:{PORT}");
    info_log(format!("Listening {IP}:{PORT}"));
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
async fn process(socket: TcpStream, _client: std::net::SocketAddr) {
    //info_log(format!("connect from {}", client));
    handle_client(socket).await;
    //info_log(format!("connect close from {}", client));
}
async fn handle_client(socket: TcpStream) {
    let mut socket = BufReader::new(socket);
    let mut buf = [0; 1024];
    //let time_begin = Instant::now();
    loop {
        let n = socket.read(&mut buf).await.expect("Buffer maybe Overflow");
        if n >= buf.len() {
            panic!("Buffer maybe Overflow")
        }
        if n == 0 {
            break;
        }
        let client_read = String::from_utf8_lossy(&buf).to_string();

        let get = format!("{}", (read_response(client_read)));
        let get = read_realget(get);
        let last_char = get.clone().pop().expect("can not get lastchar");
        let list = match fs::metadata(format!("{ROOT_DIR}{get}")) {
            Ok(n) => {
                if n.is_dir() && (last_char == '/') {
                    (format!("{ROOT_DIR}{get}index.html"), OK)
                } else if n.is_dir() && (last_char != '/') {
                    (format!("{get}/"), MOVED_PERMANENTLY)
                } else {
                    (format!("{ROOT_DIR}{get}"), OK)
                }
            }
            Err(_) => {
                wran_log(format!("2 :not found {}", get));
                (format!("{ROOT_DIR}/404.html"), NOT_FOUND)
            }
        };

        socket
            .write_all(&response(&list.0, &list.1).await)
            .await
            .expect("3 :can't write response");
    }
    /*info_log(format!(
        "response complete in {:?}",
        Instant::now() - time_begin
    ));*/
}

async fn response(url: &str, status_line: &str) -> Vec<u8> {
    //info_log(file);

    let mime_type_guess=if let Some(n) = MimeGuess::from_path(Path::new(&url)).first(){
     format!("{}", n)
    }else{"text/html".into()};

    let (contents, is_http_hender): (Vec<u8>, bool) = match status_line {
        OK | NOT_FOUND => match fs::read(url) {
            Ok(n) => (n, false),
            Err(e) => panic!("2_RDE :file can't read {url} Error is:{e}"),
        },
        MOVED_PERMANENTLY => (format!("Location: {url}\r\n\r\n").into(), true),
        _ => panic!("4: cant understand the http status_line "),
    };
    let mut response_vec = if is_http_hender {
        format!("{status_line}\r\n")
    } else {
        format!(
            "{status_line}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            mime_type_guess,
            contents.len(),
        )
    }
    .as_bytes()
    .to_vec();
    //合并http头和其余字节字节
    response_vec.extend(contents);
    response_vec
}
fn err_log<T: std::fmt::Display>(message: T) {
    //这才配得上我们最高规格的日志!!
    write_log(message, "======@#!!!!!ERROR!!!!!#@=====\n   @^ERR=>", true);
}
fn wran_log<T: std::fmt::Display>(message: T) {
    write_log(message, "WRAN", false);
}
fn info_log<T: std::fmt::Display>(message: T) {
    write_log(message, "info", false);
}
fn write_log<T: std::fmt::Display>(
    //报错详细内容
    message: T,
    //报错等级文本(用来给用户print看的，逻辑上不做限制，这里传入多花里胡哨都可以)
    //但是这个是输出到日志和终端的别搞太长了
    mode_info: &str,
    //是否使用eprint即作为错误信息输出 真即使用 假即使用print
    mode: bool,
) {
    let write_temp = format!(
        "{mode_info}[{}]:{}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        message
    );
    //作为错误信息输出或正常输出
    if mode {
        eprint!("{}", write_temp)
    } else {
        print!("{}", write_temp)
    }
    let mut binding = LOG_FILE_OPENER();
    let mut temp=binding.borrow_mut();
   
    //如果存在就写入否则忽略
    if let  &Some(ref mut n) = temp {
        write!(&mut n, "{write_temp}");
    }
}
