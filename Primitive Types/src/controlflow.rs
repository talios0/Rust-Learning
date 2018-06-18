// idk why I'm not skipping these beginning sections xD

pub fn if_statement() {
    let temp = 35;
    
    if temp > 30 {
        println!("It's pretty hot outside");
    }
    else if temp < 10 {
        println!("Cold, eh?");
    }
    else {
        println!("It's pretty nice outside");
    }

    let day = if temp > 20 { "sunny" } else {"cloudy"}; // if is an expression in Rust
    println!("Today is a {} day", day);
}