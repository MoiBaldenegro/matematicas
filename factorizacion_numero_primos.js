const example = 8

const factorizacion = (number) => {
    const processNum =  Math.round(number / 2 ) ?? 0;
    const nums = [...new Array(processNum)].map((_, index) => index + 1 );
    const filteredNums = nums.filter((i) => {
       return  number % i === 0
    });
    filteredNums.push(number);
    console.log(`Los factores del numero ${number} son:`)
    filteredNums.forEach(i => console.log(i));
    return filteredNums;
}

const isPrimo = (number) => {
   const res = factorizacion(number);
   if(res.length === 2 ) {
    console.log(`El numero ${number}, si es un numero primo.`)
   } else {
    console.log(`El numero ${number}, no es un numero primo.`)
   }
}

isPrimo(example);