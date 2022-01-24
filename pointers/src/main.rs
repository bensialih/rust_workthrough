fn main() {
    let initials = "AHB";
    let ptr_ini = &initials;

    let back_to_initial = *ptr_ini;

    let number = 8_i8;
    let another_number = number;

    let pointer = &another_number;
    let pointer_of_pointer = &pointer;
    let pointer_of_pointer_of_pointer = &pointer_of_pointer;

    println!("Hello, {:?} - with the address of {:p}", initials, initials);
    println!("pointer position is {:p}", ptr_ini);

    println!("pointer {:p} is equal to {:p}", initials, back_to_initial);

    println!(
        "number is {} and another_number is {}",
        number, another_number
    );

    println!(
        "number addr is {:p} and another_number addr is {:p}",
        &number, &another_number
    );

    println!(
        "1 {:p}, 2 {:p}, 3 {:p}",
        pointer, pointer_of_pointer, pointer_of_pointer_of_pointer
    );

    println!(
        "the value of another number is {:?}",
        ***pointer_of_pointer_of_pointer
    );
}
