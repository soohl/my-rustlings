// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec0 = Vec::new();

    // 1. let mut vec1 = fill_vec(vec0.clone());
    // 2.let mut vec1 = fill_vec2(&vec0);
    fill_vec3(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // vec1.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec2(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vect = Vec::new();

    new_vect.push(22);
    new_vect.push(44);
    new_vect.push(66);

    new_vect
}

fn fill_vec3(vec: &mut Vec<i32>) {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
}
