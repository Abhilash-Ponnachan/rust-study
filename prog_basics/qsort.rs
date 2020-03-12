fn main() {
    
    // an array of numbers (i32)
    let mut nums = [3, 2, 4, 1, 6, 5, 9];
    //let mut nums = [2, 4, 6, 3, 1, 5];
    //let mut nums = [2, 1];
    
    // qsort the array
    qsort(&mut nums);
    println!("{:?}", nums);
    
    
}

// in-place q-sort
fn qsort(ns: &mut [i32]){
    let l = ns.len();
    if l > 1{
        let mut pi = 0; // find pivot
        let mut si = pi + 1;
        while si < l {
            // scan from pivot and find smaller elements
            if ns[si] < ns[pi]{
                let t = ns[si];
                let mut j = si;
                // shift from pivot right by 1
                while j > pi{
                    ns[j] = ns[j-1];
                    j -= 1;
                }
                // swap smaller to pivot
                ns[pi] = t;
                pi += 1;
            }
            si += 1;
        }
        qsort(&mut ns[..pi]); // qsort the left (smaller) side
        qsort(&mut ns[pi+1..]); // qsort the right (greater) side
    }
}
