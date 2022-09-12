fn main() {
    main1();
    println!("");
    main2();
}

fn main1() {
    let command = "!leaderboard";
    if command == "ping" {
        println!("pong");
    } else if command == "!plant" {
        println!("planted 1 tree");
    } else if command == "!cut" {
        println!("cut 1 tree");
    } else if command == "!leaderboard" {
        println!("1 leoo#5148");
        println!("2 enzoLeDebilo#4533");
    } else {
        println!("nothing");
    }
}

fn main2() {
    let command = "!leaderboard";
    match command {
        "!leaderboard" => {
            println!("1 leoo#5148");
            println!("2 enzoLeDebilo#4533");
        }
        "!plant" => println!("planted 1 tree"),
        "!cut" => println!("cut 1 tree"),
        "ping" => println!("pong"),
        &_ => println!("nothing"),
    }
}
