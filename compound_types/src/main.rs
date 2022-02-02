fn main() {
    // Compound Types
    let tup = ("Let's get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    // let x = error_codes[3]; Gives error

    let byte = [0; 8];  // Array of length 8 with all values set to 0
}
