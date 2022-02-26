// use art::kinds::PrimaryColor;
use art::PrimaryColor;
// use art::utils::mix;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
    mix(PrimaryColor::Red, PrimaryColor::Blue);
}
