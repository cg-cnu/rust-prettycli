//! # prettycli
//!
//! `prettycli` is a collection of handy functions to print messages to terminal
//! with emojis and colors 😍
//!
//! <img src="https://user-images.githubusercontent.com/2767425/39296879-b532e770-495f-11e8-9821-fb0464126ca7.png"/> 
//! 

/// Takes a message `&str` as input and
/// Prints <p style='color:red;'>🤦 ERROR : message</p> 
///
/// # Examples
///
/// ```
/// prettycli::error("This is an error message!");
/// ``` 
pub fn error(message: &str) {
    println!("🤦  \x1B[1;31mERROR :\x1B[0m \x1B[31m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:green;'>💁 INFO : message</p> 
///
/// # Examples
///
/// ```
/// prettycli::info("This is an infomation!");
/// ``` 
pub fn info(message: &str) {
    println!("💁  \x1B[1;32mINFO :\x1B[0m \x1B[32m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:yellow;'>⚠️ WARN : message</p> 
///
/// # Examples
///
/// ```
/// prettycli::warn("This is a warning!");
/// ``` 
pub fn warn(message: &str) {
    println!("⚠️  \x1B[1;33mWARN :\x1B[0m \x1B[33m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:blue;'>🙄 WAIT : message</p> 
///
/// # Examples
///
/// ```
/// prettycli::wait("Compiling...");
/// ``` 
pub fn wait(message: &str) {
    println!("🙄  \x1B[1;34mWAIT :\x1B[0m \x1B[34m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:purple;'>🚨 CRITICAL : message</p> 
///
/// # Examples
///
/// ```
/// prettycli::critical("Are you sure, you want to remove the file ?");
/// ``` 
pub fn critical(message: &str) {
    println!("🚨  \x1B[1;35mCRITICAL :\x1B[0m \x1B[35m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:cyan;'>⚡ CMD : message</p> 
///
/// # Examples
///
/// ```
/// prettycli::command("echo $HOME");
/// ``` 
pub fn command(message: &str) {
    println!("⚡  \x1B[1;36mCMD :\x1B[0m \x1B[36m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:blue;'>🔗  LINK : </p> <p style='color:blue;text-decoration:underline'>message</p> 
///
/// # Examples
///
/// ```
/// prettycli::link("echo $HOME");
/// ``` 
pub fn link(message: &str) {
    println!("🔗  \x1B[1;34mLINK :\x1B[0m \x1B[4;34m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:grey;'>😴  MISC : message</p> 
///
/// # Examples
///
/// ```
/// prettycli::misc("echo $HOME");
/// ``` 
pub fn misc(message: &str) {
    println!("😴  \x1B[2;37mMISC :\x1B[2;37m{}\x1B[0m", message);
}
