// [x] variaveis
// [x] tipos
// [x] if, else
// [x] for, while, loop
// [x] fn
// [ ] struct, enum, generics, impl, trait

use std::collections::HashMap;
use std::fmt::{self, Display};

fn soma(x: u8, y: u8) -> u8 {
    x + y
}

fn main() {
    // let x1 = 90;
    // let x2 = 91;

    // let n = "string".to_string();

    // {
    //     // a'

    //     let x90 = n;
    // } // a'

    // println!("{}", x90);

    // let x3 = soma(x1, x2);
    // println!("{}", x3);

    // println!("{}", x1);

    // // inteiro:  i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
    // // float: f32, f64
    // // String:  &str
    // // bool: true, false
    // // char: "a"
    // // Vec<T>
    // // Map<K,V>

    // let my_name_bits: Vec<u8> = [76, 117, 99, 97, 115, 32, 22].to_vec();
    // let mut alunos: HashMap<u8, String> = HashMap::new();

    // let nome = "Lucas 👨‍💻";
    // alunos.insert(1, nome.to_string());

    // println!("Nome: {:?}", alunos);

    // let c = 10;

    // if c > 10 {
    //     println!("sim");
    // } else {
    //     println!("nao")
    // }

    // for item in 0..=1 {
    //     println!("{}", item);
    // }

    // for bit in &my_name_bits {
    //     println!("{}", bit);
    // }

    // my_name_bits.iter().for_each(|x| println!("{}", x));




    let lucas = Pessoa::new("Lucas".to_string(), 19);


    println!("{}", lucas);

}

/// # python
///
/// ```python
///
/// class Pessoa:
///     def __init__(self, nome: str, idade: int):
///         self.idade = idade
///         self.nome = nome
///
///     def niver(self):
///         self.idade += 1
/// ```
///

struct Pessoa {
    nome: String,
    idade: u8,
}

impl Pessoa {
    fn new(nome: String, idade: u8) -> Pessoa {
        Pessoa {
            nome,
            idade,
        }
    }
}


impl Display for Pessoa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pessoa(Nome: {}, Idade: {})", self.nome, self.idade)
    }
}