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

    let mut output: String = String::new();
    let mut text: String = String::new();
    match stdin().read_line(&mut text)
    {
        Ok(_) => {},
        Err(_) =>
        {
            clear_terminal();
            panic!("poruha");
        }
    };

    clear_terminal();

    for slovo_item in text.split(' ')
    {
        let mut slovo: String = String::from(slovo_item);

        let mut upper: bool = false;
        let mut contains_vowel: bool = false;

        //CUT NEW-LINE CHAR
        if text.ends_with(slovo_item)
        {
            slovo = slovo.substring(0, slovo.len() - 1).to_string();
        }

        //CHECK IF WORD STARTS WITH UPPER CASE CHAR
        if (slovo.as_bytes()[0] as char).is_uppercase() && text.starts_with(slovo_item) { upper = true; }

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
        } else if slovo.len() > 1
        {
            slovo = slovo.substring(1, slovo.len()).to_string();
        }

        slovo = slovo.to_lowercase();

        slovo = "schw".to_owned() + slovo.as_str();

        //MAKE FIRST CHAR CAPITAL IF upper
        if upper
        {
            slovo = slovo[0..1].to_uppercase() + &slovo[1..];
        }

        //ADD slovo TO output TEXT
        output = output + &slovo + " ";
    }

    //REMOVE REDUNDANT SPACE FROM output
    output = output.substring(0, output.len() - 1).to_string();

    println!("{output}");
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