pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
    costs.sort();

    let mut counter = 0;
    for x in costs {
        coins -= x;

        if coins >= 0 {
            counter += 1;
        } else {
            break;
        }
    }

    counter
}
