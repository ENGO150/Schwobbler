use std::io::stdin;

fn main()
{
    let mut slovo = String::new();
    match stdin().read_line(&mut slovo)
    {
        Ok(_) => {},
        Err(_) =>
        {
            eprintln!("poruha");
            return;
        }
    };

    println!("{slovo}");
}
