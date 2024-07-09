// Rust tiene dos tipos principales de concurrencia, threads y aysync
// CANALES (channels). Los canales en Rust proporcionan una forma de comunicación entre hilos. Es un sistema productor-consumidor

// El paquete mpsc proporciona canales para la comunicación de hilos
// mpsc significa multiple producer, single consumer
use std::sync::mpsc;
use std::thread; // módulo para trabajar con threads
use std::time::Duration; // para manerjar intervalos de tiempo

fn main() {
    // Crea un canal que consta de un transmisor (tx) y un receptor (rx)
    let (tx, rx) = mpsc::channel();

    // Se crea un hilo y ejecuta la función anónima
    // move transfiere la propiedad de las variables
    // Esto es necesario para asegurar que tenva acceso exclusivo 
    thread::spawn(move || {
        let val = String::from("hola");
        tx.send(val).unwrap(); //envia el valor a través del transmisor
        thread::sleep(Duration::from_secs(1));
    });

    let received = rx.recv().unwrap();
    println!("Recibido: {}", received);
}
