#![feature(panic_payload_as_str)]
use {
    chrono::Local,
    mime_guess::MimeGuess,
    std::{
        fs::{self, OpenOptions},
        io::Write,
        path::Path,
        sync::LazyLock,
        collections::HashMap,
    },
    tokio::{
        io::{AsyncReadExt, AsyncWriteExt,BufReader},
        net::{TcpListener, TcpStream},
    },
};
fn system_log(message:impl std::fmt::Display,mode: bool){
   let write_temp = format!(
        "[SYSTEM][{}]:{}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        message
    );
    //作为错误信息输出或正常输出
    if mode {
        eprint!("{write_temp}")
    } else {
        print!("{write_temp}")
    }
}
type LOG_FILE_OPEN<'a> = Option<std::fs::File>;
static LOG_FILE_OPENER: LazyLock<LOG_FILE_OPEN>={
    LazyLock::new(|| {
        system_log(format!("try to open the LOG file in{LOG_FILE}...."),false);
        let file_result = OpenOptions::new().append(true).open(LOG_FILE);
        if let Ok(n) = file_result {
            system_log("The log file is opened successfully",false);
            Some(n)
        } else if let Err(e) = file_result {
            system_log(format!(
                "==!!THE LOG FILE SYSTEM IS ERR CAUSE:{e}
              \n    THAT MEANS LOGS CANNOT BE RECORDED!
              \n    THIS WARNING WILL ONLY BE DISPLAYED ONCE!
              \n!!=="
            ),true);
            None
        } else {
            panic!("Undefined operation, unable to recover.")
        }
    })
};
//注意 这里使用了在nightly里不稳定的a
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
    LazyLock::force(&LOG_FILE_OPENER);
    system_log("The System Initialization and Self-Check has been Completed.",false);
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
    let mut socket=BufReader::new(socket);
    let mut buf=[0; 1024];
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
        //推荐传递使用str传到内部用string因为它不支持copy
        let get = response_reader(client_read).expect("4: RN cant read reponse");
        let get = read_realget(get.get("GET").unwrap());
        let get=get.as_str();
        
        
        let last_char = get.to_string().pop().expect("can not get lastchar");
        
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
/*fn merge_vec_u8(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    a.into_iter().chain(b.into_iter()).collect()
}*/ //旧方法
fn response_reader(response: String) -> Option<HashMap<String,String>>{
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut reponse_line =response.lines();
    let mut http_reponse_line = reponse_line.next()?.split_whitespace();
    let (http_method,resource_path,http_version)=(
      http_reponse_line.next()?,
      http_reponse_line.next()?,
      http_reponse_line.next()?
    );
    headers.insert(http_method.to_string(),resource_path.to_string());
    headers.insert("Http-Version".to_string(),http_version.to_string());
    for respose_heander in reponse_line{
      if let Some( (key,value) ) = respose_heander.split_once(": "){
             headers.insert(key.to_string(),value.to_string());
      }
    }
   Some(headers)
}
//read_response函数能正常运行就别碰纯史山旧方法
/*fn read_response(response: String) -> String {
    ((((&(((response
        .split_once('\n')
        .expect(format!("3_RD :cant read response from '{}' ", response).as_str()))
    .0)
        .to_string())[4..])
        .to_string())
    .split_once(' ')
    .unwrap())
    .0)
        .to_string()
}*/
fn read_realget(response: &str) -> String {
    if let Some(index) = response.find('?') {
        (&response[..index]).to_string()
    } else {
        response.to_string()
    }
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
    //是否使用eprintln即作为错误信息输出 真即使用 假即使用println
    mode: bool,
	){
    let write_temp = format!(
        "[{mode_info}][{}]:{}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        message
    );
    //作为错误信息输出或正常输出
    if mode {
        eprint!("{write_temp}")
    } else {
        print!("{write_temp}")
    }
    //写入日志
    if let Some(mut n)=LOG_FILE_OPENER.as_ref(){
      write!(n, "{write_temp}").expect("4: cant write to log file");
    }
}
