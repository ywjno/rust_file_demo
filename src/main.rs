use std::io::{BufRead, BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    // 正常打开文件
    let f = std::fs::File::open("Cargo.toml")?;
    println!("{:?}", f);

    // 打开文件出现错误
    // let f = std::fs::File::open("test.txt")?;
    // println!("{:?}", f);

    // 创建文件
    let mut f = std::fs::File::create("df.txt")?;
    println!("{:?}", f);
    f.write_all(b"hello world\n")?;
    f.write_all("你好\n".as_bytes())?;
    f.write_all("こんにちは\n".as_bytes())?;

    // 读取文件
    // read a file as a Vec
    let mut file = std::fs::File::open("df.txt")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    println!("{:?}", data);
    // read a file as whole as a String
    let mut file = std::fs::File::open("df.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{:?}", content);
    // read a file line by line
    let file = std::fs::File::open("df.txt")?;
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        println!("{}. {}", index + 1, line);
    }

    // 写入文件
    // 新建方式
    let mut file = std::fs::File::create("foo.txt")?;
    // write by &str
    writeln!(&mut file, "hello world")?;
    // wirte by byte
    file.write("你好\n".as_bytes())?;
    // 追加方式
    let mut file = std::fs::OpenOptions::new().append(true).open("foo.txt")?;
    writeln!(&mut file, "hello world")?;
    file.write("你好".as_bytes())?;

    // 删除文件
    std::fs::remove_file("df.txt")?;
    std::fs::remove_file("foo.txt")?;

    Ok(())
}
