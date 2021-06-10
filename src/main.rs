use std::fs::{File, write};
use std::io::stdin;
extern crate rand;
use rand::Rng;
fn main() {
    let lorem = "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";
    let mut ds = String::new();
    while ds != "n" {
        let (mut palavra, txt) = pegar_palavra();
        let corpo = ["  o\n /|\\\n / \\", "  o\n /|\\\n /", "  o\n /|\\", "  o\n /|", "  o\n /", "  o"];
        let mut vida = 0;
        let mut res: String;
        let mut act = false;

        loop {
            if vida == 6 {
                println!("vc perdeu");
                break;
            } else if !esconder(&palavra).contains('_') {
                println!("a palavra é: {}", esconder(&palavra));
                println!("vc ganhou!!!");
                if input("vc quer add uma palavra[s/n]: ") != "n" {
                    loop {
                        let add = input("add: ");
                        let add_slice: &str = &add[..];
                        print!("{}", lorem);
                        if txt.contains(add_slice) {
                            println!("essa palavra já existe em nosso jogo. tente novamente");
                        } else {
                            write("palavras.txt", format!("{},{}", txt, add)).expect("não conseguiu escrever");
                            break;
                        }
                        print!("{}", lorem);
                    }
                }
                print!("{}", lorem);
                break;
            }
            println!("vida: \n{} \npalavra: {}", corpo[vida], esconder(&palavra));
            res = input("digite uma letra: ");
            print!("{}", lorem);
            for c in  0..palavra.len() {
                if palavra[c].caracter == res && !palavra[c].visualizado {
                    act = true;
                    palavra[c].visualizado = true;
                }
            }
            if !act  {
                vida += 1;
            }
            act = false;
            
        }
        print!("{}", lorem);
        ds = input("deseja jogar novamente[s/n]: ");
        print!("{}", lorem);
    }

}
struct Letra {
    caracter: String,
    visualizado: bool,
}
fn input(msg: &str) -> String {
    eprint!("{}", msg);
    let mut out = String::new();
    stdin().read_line(&mut out).expect("err");
    out.trim().to_owned()
}
#[allow(unused_must_use)]
fn pegar_palavra() -> (Vec<Letra>, String) {
    use std::io::prelude::*;
    let mut file = File::open("palavras.txt").expect("fudeo");
    let mut palavras = String::new();
    file.read_to_string(&mut palavras);
    let vec_str = palavras.split(",").collect::<Vec<_>>();
    let rand_number = rand::thread_rng().gen_range(0, vec_str.len());
    let palavra = vec_str[rand_number].to_string();
    let mut vec_let: Vec<Letra> = Vec::new();
    for c in palavra.chars() {
        match c {
            '-' => vec_let.push(Letra {caracter: c.to_string(), visualizado: true}),
            ' ' => vec_let.push(Letra {caracter: c.to_string(), visualizado: true}),
            _ => vec_let.push(Letra {caracter: c.to_string(), visualizado: false}),
        }   
    }
    (vec_let, palavras)
}
fn esconder(arr: &Vec<Letra>) -> String {
    let mut out = String::new();
    for Letra {visualizado, caracter} in arr.iter() {
        if *visualizado {
            out.push_str(caracter);
        } else {
            out.push('_');
        }
    }
    out
}
