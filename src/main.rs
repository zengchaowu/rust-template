use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about = "文件查找工具", long_about = None)]
struct Args {
    /// 要搜索的目录路径
    #[arg(default_value = ".")]
    path: PathBuf,

    /// 要查找的文件后缀名列表（不需要包含点号，多个后缀用逗号分隔）
    #[arg(short, long)]
    extension: String,

    /// 递归搜索的最大深度（默认无限制）
    #[arg(short, long)]
    max_depth: Option<usize>,

    /// 忽略的目录名称（多个目录用逗号分隔）
    #[arg(short, long, default_value = ".git,target")]
    ignore: String,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    
    // 解析后缀名列表
    let extensions: Vec<String> = args.extension
        .split(',')
        .map(|ext| ext.trim().trim_start_matches('.'))
        .filter(|ext| !ext.is_empty())
        .map(String::from)
        .collect();

    if extensions.is_empty() {
        println!("错误：请指定至少一个文件后缀名");
        return Ok(());
    }

    // 解析忽略目录列表
    let ignore_dirs: Vec<String> = args.ignore
        .split(',')
        .map(|dir| dir.trim())
        .filter(|dir| !dir.is_empty())
        .map(String::from)
        .collect();

    // 配置遍历器
    let mut walker = WalkDir::new(&args.path);
    if let Some(depth) = args.max_depth {
        walker = walker.max_depth(depth);
    }

    // 遍历目录并查找文件
    let mut found_count = 0;
    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("警告：访问路径失败：{}", err);
                continue;
            }
        };

        // 跳过被忽略的目录
        if entry.file_type().is_dir() {
            if let Some(dir_name) = entry.path().file_name() {
                if ignore_dirs.iter().any(|ignore| ignore == dir_name.to_string_lossy().as_ref()) {
                    continue;
                }
            }
        }

        // 检查文件后缀
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                let ext_str = ext.to_string_lossy().to_string();
                if extensions.iter().any(|e| e == &ext_str) {
                    println!("{}", entry.path().display());
                    found_count += 1;
                }
            }
        }
    }

    println!("\n共找到 {} 个匹配的文件", found_count);
    Ok(())
}
