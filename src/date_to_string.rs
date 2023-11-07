use datetime::{DatePiece, LocalDate};

pub fn to_filename_string(date: &LocalDate) -> String {
    return format!(
        "{}-{}-{}",
        date.month().months_from_january() + 1,
        date.day(),
        date.year()
    );
}
