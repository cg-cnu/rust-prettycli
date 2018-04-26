pub fn error(message: &str) {
    println!("🤦  \x1B[1;31mERROR :\x1B[0m \x1B[31m{}\x1B[0m", message);
}

pub fn info(message: &str) {
    println!("💁  \x1B[1;32mINFO :\x1B[0m \x1B[32m{}\x1B[0m", message);
}

pub fn warn(message: &str) {
    println!("⚠️  \x1B[1;33mWARN :\x1B[0m \x1B[33m{}\x1B[0m", message);
}

pub fn wait(message: &str) {
    println!("🙄  \x1B[1;34mWAIT :\x1B[0m \x1B[34m{}\x1B[0m", message);
}

pub fn critical(message: &str) {
    println!("🚨  \x1B[1;35mCRITICAL :\x1B[0m \x1B[35m{}\x1B[0m", message);
}

pub fn command(message: &str) {
    println!("⚡  \x1B[1;36mCMD :\x1B[0m \x1B[36m{}\x1B[0m", message);
}

pub fn link(message: &str) {
    println!("🔗  \x1B[1;34mLINK :\x1B[0m \x1B[4;34m{}\x1B[0m", message);
}

pub fn misc(message: &str) {
    println!("😴  \x1B[2;37mMISC :\x1B[2;37m{}\x1B[0m", message);
}
