use utils::{
    switch_to_performance_core, test_both_parts_bytes, test_example_bytes,
};

fn main() {
    switch_to_performance_core();
    let result_one = test_example_bytes(14, &day14::part1, Some(21));
    let result_two = test_example_bytes(14, &day14::part2::<true>, None);
    test_both_parts_bytes(
        14,
        result_one,
        result_two,
        &day14::part1,
        &day14::part2::<false>,
    );
}
