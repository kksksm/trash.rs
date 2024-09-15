fn main(){

    let mut ingreso = String :: new();
    println!("indetifiquese: ");
    std::io::stdin().read_line(&mut ingreso).unwrap();
    reconocimiento(&mut ingreso);
    println!("bienvenido {}", ingreso);

}

fn  reconocimiento(x : &mut String){

    x.push_str("hola")
}
