enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

impl FileSize {

    fn format_size(size: u64) -> FileSize {
        match size {
            0..=999 => FileSize::Bytes(size),
            1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
            1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
            1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
            _ => FileSize::Terabytes(size as f64 / 1_000_000_000_000.0),
        }
    }

    fn display_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
            FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
            FileSize::Terabytes(tb) => format!("{:.2} TB", tb),
        }
    }
}

fn main() {
    let result = FileSize::format_size(6087375687658699);
    println!("{}", result.display_size());
}
