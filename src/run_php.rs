// use std;
// use std::process::{Command, Stdio};
// use std::fs::OpenOptions;

/*pub fn parse(source_path: &str) -> Result<String, String> {

    let child = Command::new("/bin/cat")
    .arg("file.txt")
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");

let output = child
    .wait_with_output()
    .expect("failed to wait on child");

assert!(output.status.success());
println!("{:?}", output);

Ok(String::from("ujeste te"))*/

    /*let file = OpenOptions::new().write(true).open("foo.txt").expect("není soubor");

    let command = Command::new("php")
        // .arg("-al")
        .arg(source_path)
        // .stdout(Stdio::null())
        // .stdout(Stdio.from_raw_fd(file.as_raw_fd()))
        
        .spawn();
        // .expect("failed to execute child");
        // .wait();

    
    match command {
        Ok(mut child) => {
            match child.wait() {
                Ok(value) => Ok(format!("PHP {:?}", value)),
                Err(e) => Err(format!("Error parse PHP, error {:?}", e)),
            }
            
        }
        Err(e) => Err(format!("Error parse PHP, error {:?}", e)),
    }

    
*/
    // let output = child
    //     .wait();

    // match output {
    //     Ok(_) =>  Ok(String::from("PHP")),
    //     Err(e) => Err(format!("Error parse PHP, error {:?}", e)),
    // }

    
// }