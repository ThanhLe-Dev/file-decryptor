use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

fn encrypt(path : PathBuf, lim : usize) {
    let ext = path.extension().unwrap().to_str().unwrap();
    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    
    let out_ext = match ext {
                "dng"  => "png",
                "dxt"  => "txt",
                "dpg"  => "jpg",
                "dod"  => "docx",
                _ => "このファイル種類はサーポートされない",
                };
                
    let in_path = Path::new("src").join(&path);
    let out_path = Path::new("src").join(name + "." + out_ext);
    
    print!("読み込み中 {}...", in_path.display());
    
    let mut file = match File::open(&in_path) {
        Err(why) => panic!("ファイル開けません {}: {}", in_path.display(),
                                                   why.description()),
        Ok(file) => file,
    };
    
    let mut in_data : Vec<u8> = Vec::new();
    file.read_to_end(&mut in_data).expect("データが読み込めません");
    
    
    //let out_data = in_data.iter().enumerate().map(|(i, c)| c ^ (i*28 & 0xFF) as u8).collect::<Vec<u8>>();
    let mut out_data : Vec<u8> = Vec::new();
    
    for (i,&c) in in_data.iter().enumerate() {
        if lim > 0 && i >= lim {
            out_data.push(c);
        } else {
            out_data.push(c ^ (i*28 & 0xFF) as u8);
        }
    }
    
    let mut out_file = match File::create(&out_path) {
        Err(why) => panic!("作成できません {:?}: {}",
                           out_path,
                           why.description()),
        Ok(file) => file,
    };
    
    print!(" 読み込み中...");
    out_file.write_all(&out_data);
    println!(" 完了。");
    
}

fn main() {
    let paths = fs::read_dir(&Path::new(
    &env::current_dir().unwrap()).join("src")).unwrap();
    
    let names = paths.filter_map(|entry| {
            entry.ok().and_then(|e|
                e.path().file_name()
                .and_then(|n| n.to_str().map(|s| String::from(s)))
            )
        }).collect::<Vec<String>>();
        
    for name in names {
        let fname = Path::new(&name);
        let ext = fname.extension().and_then(|n| n.to_str());
            match ext {

                 Some("dng") | Some("dxt") | Some("dpg") | Some("dod") | Some("tx") => encrypt(fname.to_path_buf(), 100),
                _ =>  println!("ファイルをスキップする {}", fname.display()),
            }
    }
    
}
