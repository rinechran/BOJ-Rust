use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn merge_sort(a: &mut Vec<i32>, p: usize, r: usize, want_index: usize, current_index: &mut usize, want_value: &mut i32) {
    if p < r {
        let q = (p + r) / 2;
        merge_sort(a, p, q, want_index, current_index, want_value);
        merge_sort(a, q + 1, r, want_index, current_index, want_value);
        merge(a, p, q, r, want_index, current_index, want_value);
    }
}

fn merge(a: &mut Vec<i32>, p: usize, q: usize, r: usize, want_index: usize,  current_index: &mut usize,  want_value: &mut i32) {
    let mut tmp = Vec::with_capacity(r - p + 1);
    let mut i = p;
    let mut j = q + 1;

    while i <= q && j <= r {
        if a[i] <= a[j] {
            tmp.push(a[i]);
            i += 1;
        } else {
            tmp.push(a[j]);
            j += 1;
        }
    }

    while i <= q {
        tmp.push(a[i]);
        i += 1;
    }

    while j <= r {
        tmp.push(a[j]);
        j += 1;
    }

    for (idx, &val) in tmp.iter().enumerate() {
        *current_index += 1;
        if *current_index == want_index {
            *want_value = val;
        }
        a[p + idx] = val;
    }
}

fn main(){
    let row: Vec<usize> = input_user_to_vec();
    let want_index = row[1];
    let mut row: Vec<i32> = input_user_to_vec();
    let mut current_index = 0;
    let mut want_value = -1;
    let len = row.len() -1;
    merge_sort(&mut row,0,len ,want_index, &mut current_index , &mut want_value );
    println!("{}",want_value);

}
