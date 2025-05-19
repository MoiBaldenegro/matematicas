fn main (){
    const EXAMPLE: i64= 7;
    isPrimo(EXAMPLE);
}

fn factorizacion(number: i64) -> Vec<i64>{
    let processNum: i64 = (number as f64  / 2.0).round() as i64 + 1;
    let nums: Vec<i64> = (1..=processNum).collect();
    let mut filteredNums:  Vec<i64> = nums.into_iter().filter(|i| number % i == 0).collect();
    println!("Los factores del numero: { } son:", number);
    filteredNums.push(number);
    filteredNums.iter().for_each(|i| println!(" {:?}", i));
    return filteredNums;
}

fn isPrimo(number: i64){
    let res : Vec<i64> = factorizacion(number); 
    if res.len() == 2 {
        println!("El numero {}, si es un número primo.", number)
    } else {
        println!("El numero {}, no es un número primo.", number)

    }
}