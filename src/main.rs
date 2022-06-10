use ofd_rust::OFD;

fn main() {
    match OFD::from_local_file("abc.ofd") {
        Ok(result) => {
            println!("create ofd using local file successfully. ofd={:?}", result)
        }
        Err(why) => {
            println!("create ofd using local file err. {}", why.to_string());
        }
    }
}
