use std::{fs::File, io::{BufWriter, Write}, path::Path};

const SDL_CONTROLLER_DB: &str = include_str!("../vendor/SDL_GameControllerDB/gamecontrollerdb.txt");

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let mut writer = BufWriter::new(File::create(Path::new(&out_dir).join("controller_db.rs")).unwrap());
    let platform = match std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => "platform:Windows,",
        "macos" => "platform:Mac OS X,",
        "ios" => "platform:iOS,",
        "linux" => "platform:Linux,",
        "android" => "platform:Android,",
        _ => {
            println!("cargo::warning=SDL_GameControllerDB is not compatible with the target OS!");
            write!(&mut writer, "static CONTROLLER_DB: &[(&str, CtrlDbEntry)] = &[];").unwrap();
            return;
        }
    };

    parse_db(platform, writer).unwrap();
}

fn parse_db(platform: &str, mut writer: BufWriter<File>) -> std::io::Result<()> {
    write!(writer, "static CONTROLLER_DB: &[(&str, CtrlDbEntry)] = &[")?;

    for line in SDL_CONTROLLER_DB.lines().filter(|x| !x.is_empty() && !x.starts_with('#')).map(|x| x.trim()) {
        if !line.ends_with(platform) {
            continue; // target OS doesn't match
        }

        let mut split = line.split(',').map(|x| x.trim()).filter(|x| !x.is_empty());
        let id = split.next().unwrap();
        let name = split.next().unwrap();
        write!(writer, "(\"{id}\", CtrlDbEntry {{ name: \"{name}\", ctrls: &[")?;
        parse_ctrls(split, &mut writer)?;
        write!(writer, "]}}),")?;
    }
    write!(writer, "];")?;
    return Ok(());
}

fn parse_ctrls<'a>(split: impl Iterator<Item = &'a str>, writer: &'a mut BufWriter<File>) -> std::io::Result<()> {
    for ctrl in split {
        let mut params = ctrl.split(':').map(|x| x.trim()).filter(|x| !x.is_empty());
        let key = params.next().unwrap();
        if key == "platform" {
            continue;
        }
        
        let value = params.next().unwrap();
        if value.starts_with("b") {
            parse_button_ctrl(key, &value[1..], writer)?;
        } else if value.starts_with("h") {
            parse_hat_ctrl(key, &value[1..], writer)?;
        } else if value.starts_with("a") {
            parse_axis_ctrl(key, &value[1..], writer)?;
        }
    }
    return Ok(());
}

fn parse_axis_ctrl(key: &str, value: &str, writer: &mut BufWriter<File>) -> std::io::Result<()> {
    if value.ends_with('~') { 
        write!(writer, "(\"{key}\",CtrlDbBinding::Axis({}, -1.0)),", value.trim_end_matches('~'))
    } else {
        write!(writer, "(\"{key}\",CtrlDbBinding::Axis({value}, 1.0)),")
    }
}

fn parse_hat_ctrl(key: &str, value: &str, writer: &mut BufWriter<File>) -> std::io::Result<()> {
    let mut split = value.split('.').map(|x| x.trim()).filter(|x| !x.is_empty());
    let left = split.next().unwrap();
    let right = split.next().unwrap();
    
    write!(writer, "(\"{key}\", CtrlDbBinding::Hat({left}, {right})),")
}

fn parse_button_ctrl(key: &str, value: &str, writer: &mut BufWriter<File>) -> std::io::Result<()> {
    write!(writer, "(\"{key}\", CtrlDbBinding::Button({value})),")
}
