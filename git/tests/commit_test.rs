#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    #[test]
    fn it_works() {
        // check if .signature() returns utc time or local time in .seconds()
        let repository = git::open_repo("tests/experiments").unwrap();
        let when = repository.blame();
        println!("When: {:?}", NaiveDateTime::from_timestamp(when.seconds(), 0));
    }
}