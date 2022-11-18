fn main () {
    match slint_build::compile("ui/test.slint") {
        Ok(_) => (),
        Err(ding) => println!("Hier ist alles kaputt, hör einfach auf, Wallah: {}", ding)
    };
}