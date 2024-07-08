
// Rust es un lenguaje de programación de sistemas diseñado por Mozilla Research que se centra en la seguridad, el rendimiento y la concurrencia. Su objetivo principal es prevenir errores de memoria y garantizar la seguridad en tiempo de compilación, sin sacrificar la velocidad.

// - Seguridad de memoria: Rust garantiza la ausencia de fallos de segmentación y condiciones de carrera mediante su sistema de propiedad, el cual se verifica en tiempo de compilación.
// - Rendimiento: Similar a C y C++, Rust se compila a código máquina, lo que permite un rendimiento muy cercano al metal.
// - Concurrencia sin miedo: Rust facilita la escritura de código concurrente seguro y eficiente, evitando problemas comunes en otros lenguajes.

// **HERRAMIENTAS**

// - Rustc: El compilador de Rust.
// - Cargo: La herramienta de construcción y gestión de dependencias de Rust.
// - Clippy: Un linter que ayuda a encontrar posibles errores y mejorar el código.
// - Rustfmt: Una herramienta de formateo de código para mantener un estilo consistente.

use std::thread;
use std::time::Duration;

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn get_value(maybe_value: Option<i32>) -> i32 {
    match maybe_value {
        Some(value) => value,
        None => 0,
    }
}

fn main() {
    println!("Hello, world!");

    // Rust es un lengauje fuertemente tipado, las variables se tienenque conocer en tiempo de compilación
    let x: i32 = 5;
    let y: u32 = 5;

    // Esto causará un error porque no se puede convertir implícitamente entre i32 y u32
    // let z: <i32 as Add<u32>>::Output = x + y;

    // Ownership. Rust garantiza la seguridad de la memoria y la ausencia de datos
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Pasamos una referencia
    println!("The length of '{}' is {}.", s1, len);

    // - String::from("hello") crea una cadena en el heap.
    // - s1 es el propietario de la cadena.
    // - calculate_length(&s1) toma una referencia a s1 sin transferir la propiedad.
    // - La cadena sigue siendo propiedad de s1 después de la llamada a calculate_length.

    // ¿Por qué pasar una referencia?

    // Para evitar la transferencia de propiedad, en Rust, 
    // pasar un valor a una función puede transferir la propiedad del valor a esa función.
    // Esto significa que el valor original ya no puede ser usado después de la llamada a la función, a menos que la función devuelva la propiedad.

    // También temas de eficiencia: Pasar referencias evita la copia de datos
    // grandes. Si String contiene una gran cantidad de datos, pasar una referencia es más eficiente que pasar una copia del valor entero.

    // Inmutabilidad y Seguridad: Al pasar una referencia inmutable (&T),
    // garantizamos que la función no modificará el valor original, lo que mejora la seguridad y la previsibilidad del código.

    // Heap. No hay que gestionar la liberación de memoria manualmente, Rust se encarga
    // Stack es una memoria de acceso rapido. Las variables locales y los argumentos de función se almacenan en el stack
    // Heap es una memoria más gande y menos estructurada para almacenar datos cuyo tamaño o vida no se conoce en tiempo de compilación
    let s = String::from("hello"); // s se almacena en el heap
    // Cuando s sale del alcance, la memoria se libera automáticamente

    // Borrowing. Se pueden referenciar datos sin transferir la propiedad utilizando &
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Las referencias permiten acceder a los datos sin tomar posesión de ellos, inmutables (&T) y mutables (&mut T)
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // Mutabilidad: Una variable mutable puede ser modificada después de su creación. Se declara con la palabra clave mut.
    let mut x = 5;
    x = 6; // Esto es válido

    // Inmutabilidad: Una variable inmutable no puede ser modificada después de su creación. Por defecto, todas las variables en Rust son inmutables.
    let x = 5;
    // x = 6; // Esto causará un error

    // Patter Matching 
    let number = 7;

    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        _ => println!("Something else!"),
    }

    // match number evalúa number.
    // Los patrones 1, 2, 3 se comparan contra number.
    // _ es un comodín que coincide con cualquier valor no manejado previamente.
    
    // Enums. Los enums permiaten definir un tipo que puede ser de varios valores y donde cada enum puede tener datos asociados
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Quit: No tiene datos asociados. Representa una variante simple.
    // Move: Tiene dos campos, x e y, ambos de tipo i32. Esta variante puede almacenar coordenadas.
    // Write: Tiene un campo de tipo String. Esta variante puede almacenar un mensaje de texto.
    // ChangeColor: Tiene tres campos, cada uno de tipo i32. Esta variante puede almacenar valores de color RGB.
    
    let msg = Message::Write(String::from("Hello"));

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
    }

    // Error Handling a través de Resulty Option
    match divide(4.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    let some_value = Some(10);
    let none_value: Option<i32> = None;

    println!("Some value: {}", get_value(some_value));
    println!("None value: {}", get_value(none_value));

    // Test
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }

        #[test]
        fn another_test() {
            assert!(true);
        }
    }
    // Traits. Es una colección de métodos que pueden ser implementadps por diferentes tipos
    trait Describable {
        fn describe(&self) -> String;
    }
    
    struct Person {
        name: String,
        age: u32,
    }
    
    impl Describable for Person {
        fn describe(&self) -> String {
            format!("{} is {} years old.", self.name, self.age)
        }
    }

    // Generics
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // let mut largest = &list[0];: Inicializa largest con una referencia al primer elemento del slice.
    // for item in list.iter(): Itera sobre cada elemento del slice.
    // if item > largest: Si el elemento actual es mayor que largest, actualiza largest para que sea una referencia a ese elemento.
    // largest: Devuelve la referencia al elemento mayor encontrado.

    // Threads
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

}
