use clap::Parser;

#[derive(Parser)]
struct CLI {
    string: String,
    rot: usize 
}

fn main() {
    let args = CLI::parse();
    let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let upper_alphabet = alphabet
        .iter()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<_>>();
    let n = args.rot;
    let rot = args
        .string
        .chars()
        .map(|c| *alphabet.iter()
             .chain(alphabet.iter())
             .chain(upper_alphabet.iter())
             .chain(upper_alphabet.iter())
             .skip_while(|&x| *x != c)
             .nth(n)
             .unwrap_or(&c)
             )
        .collect::<String>();
    println!("{:?}", rot);
}
