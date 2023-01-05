use std::io::{stdout, Write};

use subprocess::{CaptureData, Exec, Redirection};

pub fn run<S: Into<String>>(cmd: S) -> Result<CaptureData, CaptureData> {
    let cmd = cmd.into();
    println!("running: {:#?}", cmd);
    stdout().flush();
    let _message = "doing work".to_string();
    // let sp = Spinner::new(Spinners::Dots, message);
    let ret = match Exec::shell(cmd).stdout(Redirection::Pipe).capture() {
        Ok(v) if v.exit_status.success() => Ok(v),
        Ok(v) => Err(v),
        Err(_) => panic!("unknown exit_status"),
    };
    // sp.stop_with_symbol("💁‍♀️");
    ret
}
