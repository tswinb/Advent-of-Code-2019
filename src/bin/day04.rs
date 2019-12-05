fn main() {

    puzzle_1();
    puzzle_2();

}

fn puzzle_1() {
    let mut count = 0;

    for i in 271973..=785961 {

        let x = i.to_string();

        let contains_doubles = x.as_bytes().windows(2).any(|x| x[0] == x[1]);
        let no_increasing = x.as_bytes().windows(2).all(|x| x[0] <= x[1]);

        if contains_doubles && no_increasing {
            count = count + 1;
        }
    }
    println!("Puzzle 1: {}", count);
}

fn puzzle_2() {
    let mut count = 0;
    for i in 271973..=785961 {
        let x = i.to_string();
        let v: Vec<&str> = x.split("").collect();

        let double_case_1 = v.windows(4).any(|x| x[0] != x[1] && x[1] == x[2] && x[2] != x[3]);
        // case when at start of number ->
        let double_case_2 = v[0] == v[1] && v[1] != v[2];
        // case when at end of number ->
        let double_case_3 = v[v.len()-1] == v[v.len()-2] && v[v.len()-2] != v[v.len()-3];

        let no_increasing = x.as_bytes().windows(2).all(|x| x[0] <= x[1]);

        if (double_case_1 || double_case_2 || double_case_3) && no_increasing {
            count = count + 1;
        }
    }
    println!("Puzzle 2: {}", count);
}
