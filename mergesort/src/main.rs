fn mergesort(arr: &Vec<i32>) -> Vec<i32> {
    if arr.len() == 1{
        return vec!(arr[0])
    } else {
        let mid_size : usize = arr.len()/2;
        return merge(&mergesort(&arr[..mid_size].to_vec()), &mergesort(&arr[mid_size..].to_vec()))
    }
}

fn merge(arr1 : &Vec<i32>, arr2 : &Vec<i32>) -> Vec<i32> {
    let mut ret_vector : Vec<i32> = Vec::new();

    let (mut pivo1, mut pivo2) : (usize, usize) = (0, 0);

    let mut choose_value = |vector :&Vec<i32>, pivo :usize| {
        ret_vector.push(vector[pivo]);
        pivo + 1
    };

    while pivo1 < arr1.len() && pivo2 < arr2.len() {
        if arr1[pivo1] < arr2[pivo2] {
            pivo1 = choose_value(arr1, pivo1);
        } else {
            pivo2 = choose_value(arr2, pivo2);
        }
    }

    while pivo1 < arr1.len() {
        pivo1 = choose_value(arr1, pivo1);
    }
    while pivo2 < arr2.len() {
        pivo2 = choose_value(arr2, pivo2);
    }

    return ret_vector
}


fn main() {
    let teste_v = vec!(5, 2, 3, 4, 1);
    println!("{:?}", mergesort(&teste_v));
}
