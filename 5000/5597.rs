use std::io;

fn main(){
    let mut number_array : Vec<i32> = Vec::new();

    for i in 0..30{
        number_array.push(0);
    }

    for i in 0..28{
        let mut number_line = String::new();

        io::stdin().read_line(&mut number_line).unwrap();

        let input_number : usize = number_line
            .trim()
            .parse()
            .unwrap();

        number_array[input_number-1] = 1;
    }

    for i in 0..30{
        if number_array[i]==0{
            println!("{}",i+1);
        }
    }
    
}