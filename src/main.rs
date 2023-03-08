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

fn main()
{
    clear_terminal();
    let mut slovo: String = String::new();
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

    slovo = slovo.substring(1, slovo.len() - 1).to_string();

    slovo = "schw".to_owned() + slovo.as_str();

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