fn main(){
    let mut a: String = String::from("Hola");
    println!("value of {} is {}", 'a', a);

    let b = &mut a;
    b.push('!');
    println!("value of {} is {}", 'b', b);


    let c = a;
    println!("value of {} is {}", 'c', c);
}