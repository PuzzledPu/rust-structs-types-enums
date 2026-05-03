fn add_to_ends(v: &mut Vec<i32>, element: i32) -> &Vec<i32> {
    v.insert(0, element); // Add to the beginning
    v.push(element);
    v // Add to the end
}

fn combine_vectors(v1: &mut Vec<i32>, v2: Vec<i32>) -> &Vec<i32> {
    v1.extend(v2);
    v1 // Add all elements of v2 to v1
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    println!("{:?}", add_to_ends(&mut v, 42));
    println!("{:?}", combine_vectors(&mut v, vec![71, 97]));
}
