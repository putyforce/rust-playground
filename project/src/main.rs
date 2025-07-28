fn main() {
    print_labeled_measurement(33, 'g');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("protein scope: {value}{unit_label}");
}
