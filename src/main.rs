use std::
{
    io::
    {
        stdin,
        stdout,
        Write,
    },
};

use substring::Substring;
use is_vowel::IsRomanceVowel;

fn main()
{
    clear_terminal();

    let mut slovo: String = String::new();
    let mut upper: bool = false;
    let mut contains_vowel: bool = false;
    match stdin().read_line(&mut slovo)
    {
        Ok(_) => {},
        Err(_) =>
        {
            clear_terminal();
            panic!("poruha");
        }
    };

    clear_terminal();

    //CUT NEW-LINE CHAR
    slovo = slovo.substring(0, slovo.len() - 1).to_string();

    //CHECK IF WORD STARTS WITH UPPER CASE CHAR
    if (slovo.as_bytes()[0] as char).is_uppercase() { upper = true; }

    //LOAD contains_vowel
    for c in slovo.chars()
    {
        if c.is_romance_vowel()
        {
            contains_vowel = true;
            break;
        }
    }

    //REMOVE ANY UNWANTED PREFIX
    if contains_vowel
    {
        while !(slovo.as_bytes()[0] as char).is_romance_vowel()
        {
            slovo = slovo.substring(1, slovo.len()).to_string();
        }
    } //TODO: else

    slovo = slovo.to_lowercase();

    slovo = "schw".to_owned() + slovo.as_str();

    //MAKE FIRST CHAR CAPITAL IF upper
    if upper
    {
        slovo = slovo[0..1].to_uppercase() + &slovo[1..];
    }

    println!("{slovo}");
}

fn clear_terminal()
{
    print!("\x1B[2J\x1B[1;1H");

    match stdout().flush()
    {
        Ok(_) => {},
        Err(_) => panic!("poruha"),
    };
}