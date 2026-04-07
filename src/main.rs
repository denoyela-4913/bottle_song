pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    //todo!("Return the bottle song starting at {start_bottles} and taking down {take_down} bottles")
    for bottle in (start_bottles - take_down + 1..=start_bottles).rev() {
        // Debug
        //println!("bottle = {bottle}");
        song.push_str(&verse(bottle));
        if bottle != start_bottles - take_down + 1 {
            song.push('\n');
        }   
    }
    song
} // fn recite

fn verse (bottle: u32) -> String {
    let mut text = String::new();

    let cur_n_str: String = match bottle {
        0 => "no".to_string(),
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        _ => bottle.to_string()
    };
    let prev_n_str: String = match bottle - 1 {
        0 => "no".to_string(),
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        _ => bottle.to_string()
    };

    // Specific case : One
    if bottle == 1 {
        let s = format!("{cur_n_str} green bottle hanging on the wall,\n");
        text.push_str(&s);
        text.push_str(&s);
        // Debug
        // print!("{s}");
        // print!("{s}");
    } else {
        let s = format!("{cur_n_str} green bottles hanging on the wall,\n");
        text.push_str(&s);
        text.push_str(&s);
        // Debug
        // print!("{s}");
        // print!("{s}");
    }
    let s = "And if one green bottle should accidentally fall,\n".to_string();
    text.push_str(&s);
    // Debug
    // print!("{s}");

    // Specific case : Two => One bottle left, is hanging on the wall
    if bottle == 2 {
        let s = format!("There'll be {} green bottle hanging on the wall.\n", prev_n_str.to_lowercase());
        text.push_str(&s);
        // Debug
        // print!("{s}");
    } else {        
        let s = format!("There'll be {} green bottles hanging on the wall.\n", prev_n_str.to_lowercase());
        text.push_str(&s);
        // Debug
        // print!("{s}");
    }
    
    text
} // fn verse


fn main() {
    println!("Hello, world! bottle song");
    let text = recite(10,3);
    // Debug
    println!("\n|{text}|");
}
