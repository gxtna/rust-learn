use std::env;
use minigrep::Config;
use std::process;

fn main() {
    // 读取命令行参数
    // env::args 会产生一个迭代器
    // unwrap_or_else 产生一个闭包，打印错误信息，process 传入状态码退出程序
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing argumnets:{}",err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error:{}",e);
        process::exit(1)
    }
}
