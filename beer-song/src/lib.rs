pub fn verse(n: u32) -> String {
    let current_beers = n;
    let taken_beer = 1;
    let mut beers_left =0;

    if current_beers > 1{
        beers_left = n - taken_beer;
    }

    let mut verse_sang = String::new();
  
    if n == 1{
        verse_sang =  format!( "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    } else if n == 0 {
        verse_sang = format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }else {

        if beers_left == 1 {
            verse_sang  = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", current_beers, current_beers, beers_left);

        }else {
            verse_sang  = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", current_beers, current_beers, beers_left);
        }
    }
   

    return verse_sang
}
pub fn sing(start: u32, end: u32) -> String {
    let mut concatenation = String::new();

    for verses in (end..=start).rev() {
        let new_verse = verse(verses);
        concatenation = format!("{}\n{}", concatenation, new_verse);
    }

    concatenation = concatenation.trim_start().to_string();

    concatenation
}
