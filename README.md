#miser
##一个简易的http服务器
###配置环境
####请先确保你在使用nightly版编译器
切换nightly版
```bash
rustup default nightly
###手动编译
```bash
cargo build --release
```
* 需知
> 编译后的执行文件在 target/release里
> 运行时确保程序有权限访问 "/var/log/minser/connect.log" 和 /server
###部署和编译
```bash
bash setup.sh
```

* 本程序应该只尝试过在linux运行
>默认网站根目录为/server
>默认页面index.html
>默认404页面404.html
>默认日志/var/log/minser/connect.log

##待办
- [ ] 允许不严格url访问
- [ ] 使用serde_ymal来配置而不是在代码里修改
- [ ] 代码可读性优化 
- [x] 优化readme为md
- [ ] 兼容windows
