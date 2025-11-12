// Closure in rust
// closure in rust is basically an anonymous function , that is function that can be created without giving it a name



fn main() {
   let x=5;
   let y=20;
   let add_x =|num: i32| num + x;
//    let multiply= |a:i32, b:i32| a*b;

let multiply = |a:i32, b:i32| -> i32{
    a*b
};
  
  // Closures and Ownership


  count();
}

fn add_y(x:i32, y: i32) -> i32{
     return x + y;
} 

fn count(){
    let mut count = 1;

    let mut increment=|| {
        count += 1;
        println!("Count is {}", count);
    };

    increment();
    increment();
}
