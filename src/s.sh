#!/bin/bash

# 模拟定义一些常量（这里只是简单的变量赋值，和Rust里的静态常量概念不完全相同）
IP="0.0.0.0"
PORT="25565"
OK="HTTP/1.1 200 OK"
NOT_FOUND="HTTP/1.1 404 Not Found"
ROOT_DIR="/server"
LOG_FILE="/var/log/minser/connect.log"

# 处理客户端请求的函数
handle_client() {
    local request="$1"
    local get_path=$(echo "$request" | sed 's/^.*GET \([^ ]*\).*$/\1/' | sed "s/^\///")
    local file_path="$ROOT_DIR/$get_path"
    if [ -f "$file_path" ]; then
        local status_line="$OK"
        local content=$(cat "$file_path")
    elif [ -d "$file_path" ]; then
        local status_line="$OK"
        local content=$(cat "$file_path/index.html")
    else
        local status_line="$NOT_FOUND"
        local content="404 Page Not Found"
        echo "2 :not found $file_path" | tee -a "$LOG_FILE"
    fi
    local response="$status_line\r\nContent-Length: ${#content}\r\n\r\n$content"
    echo -e "$response"
}

# 处理连接的函数
handle_connection() {
    # 使用netcat来模拟监听端口接收连接（这里是简单替代，实际可能需要更复杂配置来达到类似TcpListener功能）
    while read -r line; do
        client_address=$(echo "$line" | cut -d' ' -f1)
        echo "connect from $client_address" | tee -a "$LOG_FILE"
        handle_client "$line"
        echo "connect close from $client_address" | tee -a "$LOG_FILE"
    done < <(nc -l -p "$PORT")
}

# 启动服务的主函数逻辑，这里用无限循环来模拟持续监听
while true
do
    # 模拟接受连接并处理，这里简单地在后台启动一个处理函数（模拟原Rust代码里的Tokio spawn等操作）
    ( handle_connection ) &
    # 可以适当添加一些延时或者控制并发连接数量等逻辑，这里简单睡眠一下，避免过度消耗资源
    sleep 0.1
done
