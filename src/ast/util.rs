pub fn new_label_id() -> usize {
    static mut LABEL_COUNT: usize = 1;

    unsafe {
        let id = LABEL_COUNT;
        LABEL_COUNT += 1;
        id
    }
}

pub fn new_temp_id() -> usize {
    static mut TEMP_COUNT: usize = 0;

    unsafe {
        let id = TEMP_COUNT;
        TEMP_COUNT += 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_ids() {
        assert_eq!(1, new_label_id());
        assert_eq!(2, new_label_id());
        assert_eq!(3, new_label_id());
        assert_eq!(4, new_label_id());
    }

    #[test]
    fn temp_ids() {
        assert_eq!(0, new_temp_id());
        assert_eq!(1, new_temp_id());
        assert_eq!(2, new_temp_id());
        assert_eq!(3, new_temp_id());
    }
}
