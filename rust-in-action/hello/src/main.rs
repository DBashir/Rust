fn greet_world() {
    println!("hello world");
    let southern_germany = "Grub Gott!";
    let japan = "1402141204";
    let regions = [southern_germany, japan];
    for region in regions.iter()
    {
        println!("{}", &region);
    }
}



fn main(){
    greet_world();
}