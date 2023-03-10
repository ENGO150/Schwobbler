use std::io::
{
    stdin,
    stdout,
    Write,
};

use std::env::args;

use substring::Substring;

trait Vowel
{
    fn is_vowel(&self) -> bool;
}

impl Vowel for char
{
    fn is_vowel(&self) -> bool
    {
        "aeiouáéěíóůú".to_string().contains(self.to_owned().to_ascii_lowercase())
    }
}

fn main()
{
    clear_terminal();

    let mut args: Vec<String> = args().collect();
    let mut output: String = String::new();
    let mut text: String;

    //REMOVE PATH FROM args
    args.remove(0);

    if args.is_empty()
    {
        text = String::new();

        //GET INPUT
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
    } else //INPUT FROM CONSOLE
    {
        text = args.join(" ") + "\n";
    }

    for slovo_item in text.split(' ')
    {
        let mut slovo: String = String::from(slovo_item);

        let mut upper: bool = false;
        let mut contains_vowel: bool = false;

        //CHECK IF WORD STARTS WITH UPPER CASE CHAR
        if (slovo.as_bytes()[0] as char).is_uppercase() && text.starts_with(slovo_item) { upper = true; }

        //LOAD contains_vowel
        for c in slovo.chars()
        {
            if c.is_vowel()
            {
                contains_vowel = true;
                break;
            }
        }

        //REMOVE ANY UNWANTED PREFIX
        if contains_vowel || slovo.len() > 1
        {
            if contains_vowel
            {
                while !(slovo.as_bytes()[0] as char).is_vowel()
                {
                    slovo = slovo.substring(1, slovo.len()).to_string();
                }
            } else { slovo = slovo.substring(1, slovo.len()).to_string(); } //TODO: Do something with the repetitive code

            slovo = "schw".to_owned() + slovo.as_str();
        }

        slovo = slovo.to_lowercase();

        //MAKE FIRST CHAR CAPITAL IF upper
        if upper
        {
            slovo = slovo[0..1].to_uppercase() + &slovo[1..];
        }

        //ADD slovo TO output TEXT
        output = output + &slovo + " ";
    }

    //REMOVE REDUNDANT SUFFIX FROM output
    output = output.substring(0, output.len() - 2).to_string();

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