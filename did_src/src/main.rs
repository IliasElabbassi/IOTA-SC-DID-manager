mod create_did; // A mod declaration makes the Rust compiler look for the corresponding .rs files automatically!
use did_src::PKG_NAME;
use std::io;
use std::env;

// fn help(){
//     println!("Here is a list of the different options you get : ");
//     println!("- create [available] [create a DID]");
//     println!("- delete [not available] [delete a DID]");
//     println!("- VM [not available] [Add a Verification Method]");
//     println!("- VC [not available] [Create Verifiable Credential]");
//     println!("- more to come... ");
//     println!("");
//     println!("- stop [to stop the script]")
// }

fn main(){
    let args: Vec<_> = env::args().collect();
    if args.len() < 1 {
        println!("You should pass a password to the script !");
    }
    // println!("{}",args[1]);

    // println!("Welcome to your Dicentralized Identity platform !!!\n");
    // help();

    // let stdin = io::stdin();
    // let mut user_input = String::new();
    // let mut stop = false;

    // while !stop{
    //     stdin.read_line(&mut user_input);
    //     if user_input.contains("stop"){
    //         println!("stoping script...");
    //         stop = true;
    //     } else if user_input.contains("create"){
    //         println!("creating a did :");
    //         create_did::create(); // Use :: to Call a function defined in the other file (module)
    //         println!("did created !");
    //     }else {
    //         println!("command not recognized !");
    //     }
    // }
    create_did::create( &args[1]); // Use :: to Call a function defined in the other file (module)
    println!("did created !");
}