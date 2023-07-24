// Destructure the `cat` tuple so that the println will work.

fn main() {

    let cat = ("Furry McFurson", 3.5);
    let /* Your pattern here */(name, age) = cat;

    println!("{} is {} years old.", name, age);
}
