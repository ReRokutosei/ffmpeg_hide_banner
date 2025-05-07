use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    // 获取当前可执行文件的目录路径
    let current_exe = env::current_exe().expect("Failed to get current exe path");
    let exe_dir = current_exe.parent().expect("Failed to get exe directory");
    
    // 构建 ffprobe_orig.exe 的完整路径
    let mut ffprobe_path = PathBuf::from(exe_dir);
    ffprobe_path.push("ffprobe_orig.exe");

    // 获取命令行参数
    let args: Vec<String> = env::args().skip(1).collect();
    
    // 构建命令
    let mut command = Command::new(ffprobe_path);
    command.arg("-hide_banner");
    command.args(args);

    // 执行命令并等待完成
    let status = command
        .spawn()
        .expect("Failed to execute ffprobe")
        .wait()
        .expect("Failed to wait for ffprobe");

    // 使用相同的退出码退出
    std::process::exit(status.code().unwrap_or(1));
}