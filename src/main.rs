use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, Result, Write, BufReader};

enum Options {
    FizzBuzz,
    CharacterCounter,
    NumberClassifier,
    Exit,
}

trait LabI {
    fn fizz_buzz(&self);
    fn character_counter(&self);
    fn number_classifier(&self)-> Result<()>;
}

struct ImplementationLabI;
impl LabI for ImplementationLabI {

    fn fizz_buzz(&self) {
        println!("Ingrese un número para FizzBuzz:");
        let num: i32 = input_user_fizzbuzz();
        let mut results = Vec::new();

        for i in 1..=num{
            if i % 3 == 0 && i % 5 == 0 {
                results.push("FizzBuzz".to_string());
            } else if i % 3 == 0{
                results.push("Fizz".to_string());
            } else if i % 5 == 0 {
                results.push("Buzz".to_string());
            } else {
                results.push(i.to_string());
            }
        }
    
        for chunk in results.chunks(10){
            println!("{}", chunk.join("\t"));
        }
        println!();
    }


    fn character_counter(&self) {
        println!("Ingrese un string para analizar sus caracteres:");
        let input_string = input_user_string();
        let (digits, vowels, consonants, symbols) = character_analyzer(&input_string);

        println!("Resultados:");
        println!("Digit: {}",digits);
        println!("Vowels: {}", vowels);
        println!("Consonants: {}",consonants);
        println!("Symbols: {}",symbols);
        println!();
    }

    
    fn number_classifier(&self) -> Result<()>{
        let input_file = File::open("numbers-input.txt")?;
        let mut output_file = File::create("numbers-output.txt")?;

        let content = BufReader::new(input_file);
        for line in content.lines() {
            let var_line = line.unwrap();
            let line = var_line.trim();
            let number = line.parse::<u32>().unwrap_or_default();
            let classified_number = classifier(number);
            let modified_line = format!("{}  -  {}",line, classified_number);
            writeln!(output_file, "{}",modified_line)?;
        }      
        println!("Proceso completado. \n");
        Ok(())
    } 
}


fn classifier(num: u32) -> String {
    if num %2 ==0 && is_prime(num) {
        "Prime Even".to_string()
    } else if num %2 == 0 {
        "Even".to_string()
    } else if is_prime(num) {
        "Prime".to_string()
    } else {
        "_".to_string()
    }
}


fn is_prime(num:u32)->bool{
    if num <= 1 {
        return false;
    }
    for i in 2..=(num / 2){
        if num % i == 0 {return false;}
    }
    return true;
}


fn input_user_fizzbuzz() ->i32{
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    input.trim().parse().expect("Entrada no valida")
}


fn input_user_string() ->String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    input.trim().to_string()
}


fn character_analyzer(input: &str) -> (usize, usize, usize, usize){
    let (mut digits, mut vowels, mut consonants, mut symbols) =(0,0,0,0);

    for character in input.chars() {
        if character.is_digit(10) {
            digits += 1;
        } else if character.is_alphabetic() {
            if "AEIOUaeiou".contains(character){
                vowels += 1;
            } else {
                consonants+= 1;
            }
        } else {symbols+= 1;}
    }
    (digits, vowels, consonants, symbols)
}


fn menu(option: Options, lab: &dyn LabI) -> bool{
    match option{
        Options::FizzBuzz =>{
            lab.fizz_buzz();
            true
        }
        Options::CharacterCounter =>{
            lab.character_counter();
            true
        }
        Options::NumberClassifier => {
            let _= lab.number_classifier();
            true
        }
        Options::Exit =>{
            println!("Programa finalizado");
            false
        }
    }
}


fn display_menu(){
    println!("Seleccione una opción:");
    println!("1. FizzBuzz");
    println!("2. Contador de vocales, consonantes, números y símbolos");
    println!("3. Formateador de números: lectura y escritura de archivos");
    println!("4. Salir");
}


fn input_user() -> Options{
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    match input.trim().parse().unwrap_or(4) {
        1 => Options::FizzBuzz,
        2 => Options::CharacterCounter,
        3 => Options::NumberClassifier,
        _ => Options::Exit,
    }
}


fn main() ->Result<()> {
    let lab = ImplementationLabI;
    let mut show_menu: bool=true;
    
    while show_menu{
        display_menu(); 
        stdout().flush()?;

        let choice: Options = input_user();
        show_menu =menu(choice, &lab);
    }

    Ok(())
}