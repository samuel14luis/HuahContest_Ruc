use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

fn main() {
    let filename = "src/input.txt";
    // Abrir archivo en modo read-only (ignorando errores).
    let input = File::open(filename).unwrap();
    let mut reader = BufReader::new(input);

    // Write a &str in the file (ignoring the result).
    //writeln!(&mut file, "Hello World!").unwrap();

    // Write a byte string.
    //file.write(b"Bytes\n").unwrap();

    // Leer linea a linea utilizando el iterador lines() de std::io::BufRead.
    let mut linea = String::new();
    reader.read_line(&mut linea).expect("Error al obtener primera línea");
    println!("vueltas en el for: {} ", linea);

    let mut output = File::create("src/output.txt").unwrap();

    for (i, ruc) in reader.lines().enumerate() {
        let ruc:String = ruc.unwrap();        
        let resultado = analizar_ruc(ruc);
        writeln!(&mut output, "{}", resultado).unwrap();
        println!("{}. {}", i + 1, resultado);        
    }
}

fn analizar_ruc(ruc:String) -> &'static str {
        let mut suma:u32 = 0;
        let resto:u32;
        let complemento:u32;
        let mut last_digit:u32 = 0;
        //Paso 1 y 2
        for (j, n) in ruc.chars().enumerate() {
            let n:u32 = n.to_digit(10).unwrap();
            let aux:f32;
            if j < 4 {
                aux = ( -0.625*(j+1) as f32 + 3.75)/0.625;
                suma += n*aux as u32;
                //println!("   -> {}. {}x{} = {}", j + 1, aux, n, n*aux as u32); //y = (- 0.625x + 3.75)/0.625
                
            } else if j < 10{
                aux = (- 0.546875*(j+1) as f32 + 6.5625)/0.546875;
                suma += n*aux as u32;
                //println!("   -> {}. {}x{} = {}", j + 1, aux, n, n*aux as u32); //y = (- 0.546875x + 6.5625)/0.546875
            } else {
                last_digit = n;
                //println!("   ->       {}. {}", j + 1, n);
            }
            //println!("   -> {}. {}", j + 1, n);
        }
        //let ruc:u64 = ruc.unwrap().parse().unwrap();
        //Calcular resto y complemento
        resto = suma%11;
        complemento = 11 - resto;
        if last_digit == complemento % 10 {
            return "YES";
        } else {
            return "NO";
        }
}