use utils::{switch_to_performance_core, test_both_parts_bytes, test_example_bytes};

fn main() {
    switch_to_performance_core();
    let result_one = test_example_bytes(9, &day9::part1, Some(1928));
    let result_two = test_example_bytes(9, &day9::part2, Some(2858));
    test_both_parts_bytes(9, result_one, result_two, &day9::part1, &day9::part2);
}
