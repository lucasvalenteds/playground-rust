#[cfg(test)]
mod tests {

    use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
    use std::fs as FileSystem;

    #[test]
    fn persisting_and_reading_string() {
        // Setup
        let filename = Box::new(format!("{}/example1.db", env!("XDG_RUNTIME_DIR")));
        let mut database = PickleDb::new(
            filename.as_ref(),
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        );

        // Arrange
        let (key, value) = ("name", "John Smith");

        // Act
        database.set(&key, &value).unwrap();
        let name = database.get::<String>(&key).unwrap();

        // Assert
        assert_eq!("John Smith", name);
        assert_eq!(true, database.exists(&key));

        // Tear down
        FileSystem::remove_file(filename.as_ref()).unwrap();
    }

    #[test]
    fn persisting_and_removing_string() {
        // Setup
        let filename = Box::new(format!("{}/example2.db", env!("XDG_RUNTIME_DIR")));
        let mut database = PickleDb::new(
            filename.as_ref(),
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        );

        // Arrange
        let (key, value) = ("id", "715d04ad-a23f-4746-bfbc-7d8b33339583");
        database.set(&key, &value).unwrap();

        // Act
        let remove_key_with_value = database.rem(&key).unwrap();
        let remove_key_without_value = database.rem(&key).unwrap();
        let remove_unknown_key = database.rem("unset").unwrap();

        // Assert
        assert_eq!(true, remove_key_with_value);
        assert_eq!(false, remove_key_without_value);
        assert_eq!(false, remove_unknown_key);

        // Tear down
        FileSystem::remove_file(filename.as_ref()).unwrap();
    }
}
