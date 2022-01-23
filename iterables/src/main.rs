trait TypeInfo {
    fn get_type(&self) -> &'static str;
}

impl<T> TypeInfo for T {
    fn get_type(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

// into_iter : create a completely new iterator - i32
// iter: iterator that points to the original variable &i32
// iter_mut: &mut i32

struct Person {
    name: &'static str,
    n: i32,
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let chars: std::vec::Vec<String> = self
            .name
            .split("")
            .map(|x| format!("{}", x))
            .filter(|x| x != &String::from(""))
            .collect();

        chars.into_iter()
    }
}

fn main() {
    let mut person = Person {
        name: "Abdel",
        n: 0,
    };

    let my_iter = person.into_iter();

    println!(
        "this is the n value: {:?}\n\n the type of this is {:?}",
        my_iter,
        my_iter.get_type()
    );

    for i in my_iter {
        println!("char > {} with type {}", i, i.get_type());
    }

    // let mut numbers = vec![1, 3, 5, 7];
    // println!("type >>> {}", numbers.get_type());

    // let mut my_new_iter = numbers.iter();

    // let first_value = my_new_iter.next();
    // println!("this variable should be &i32 {}", first_value.get_type());
    // // println!("{:?}", numbers);

    // let mut my_new_mut_iter = numbers.iter_mut();
    // println!(
    //     "this variable should be &mut i32 {}",
    //     my_new_mut_iter.next().get_type()
    // );

    // let mut get_my_into_iter = numbers.into_iter();

    // let new_first = get_my_into_iter.next();
    // println!("this value should be i32 > {}", new_first.get_type());
}
