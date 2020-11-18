pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += num_read;
    if !silent {
        eprintln!("\r{}", total_bytes);
        if last {
            eprintln!();
        }
    }
}
