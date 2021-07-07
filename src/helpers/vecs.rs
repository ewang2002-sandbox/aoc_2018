/// Finds the intersection of two vectors.
///
/// # Arguments
///
/// * `first_vec` - The first vector.
/// * `second_vec` - The second vector.
///
/// # Returns
///
/// * The intersection of those vectors.
pub fn find_intersection<T>(first_vec: &Vec<T>, second_vec: &Vec<T>) -> Vec<T>
    where T: Eq + Clone {
    let mut return_vec: Vec<T> = Vec::new();
    for elem in first_vec {
        if second_vec.contains(&elem) {
            return_vec.push(elem.clone());
        }
    }

    return return_vec;
}

pub fn num_intersections<T>(a: &Vec<T>, b: &Vec<T>) -> u32 where T: Eq {
    let mut num = 0;
    for elem in a {
        if b.contains(&elem) {
            num += 1;
        }
    }

    return num;
}

pub fn has_intersection<T>(a: &Vec<T>, b: &Vec<T>) -> bool where T: Eq {
    let mut num = 0;
    for elem in a {
        if b.contains(&elem) {
            return true;
        }
    }

    return false;
}