struct ParkingSystem {
    parking_map: std::collections::HashMap<i32, (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        let mut parking_map = std::collections::HashMap::new();

        parking_map.insert(1, (0, big));
        parking_map.insert(2, (0, medium));
        parking_map.insert(3, (0, small));

        Self { parking_map }
    }

    fn add_car(mut self, car_type: i32) -> bool {
        let mut result = false;

        self.parking_map
            .entry(car_type)
            .and_modify(|(current, max)| {
                if current < max {
                    *current += 1;
                    result = true;
                }
            });

        result
    }
}
