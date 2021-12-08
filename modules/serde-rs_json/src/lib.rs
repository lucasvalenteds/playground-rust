use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub code: String,
    pub version: i32,
    pub active: bool,
    pub owner: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::Device;
    use serde_json as Serde;

    #[test]
    fn serializing_and_deserializing_vector() {
        // Arrange
        let device_1 = Device {
            code: String::from("D1"),
            version: 1,
            active: true,
            owner: Some(String::from("John Smith")),
        };
        let device_2 = Device {
            code: String::from("D2"),
            version: 5,
            active: false,
            owner: None,
        };
        let devices = vec![device_1.clone(), device_2.clone()];

        // Act
        let serialized: String = Serde::to_string(&devices).unwrap();
        let deserialized: Vec<Device> = Serde::from_str(&serialized).unwrap();

        // Assert
        let device_1_json =
            "{\"code\":\"D1\",\"version\":1,\"active\":true,\"owner\":\"John Smith\"}";
        let device_2_json = "{\"code\":\"D2\",\"version\":5,\"active\":false,\"owner\":null}";
        let devices_json = format!("[{},{}]", device_1_json, device_2_json);

        assert_eq!(devices_json, serialized);
        assert_eq!(devices.len(), deserialized.len());
        assert_eq!(devices, deserialized);
    }
}
