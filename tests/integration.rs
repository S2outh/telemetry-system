use telemetry_system::*;

#[derive(TMValue, Default, Clone, Copy)]
struct TestValue {
    val: u32
}

#[derive(TMValue, Default, Clone, Copy)]
struct TestVector {
    x: i16,
    y: f32,
    z: TestValue
}


#[tm_definition_macro_attribute]
mod telemetry {
    #[tmv(u32, id = 12, address = "first_value")]
    struct FirstTMValue;
    #[tmv(TestValue, id = 1)]
    struct SecondTMValue;
    mod some_other_mod {
        #[tmv(ThirdTMValue, id = 3)]
        struct ThirdTMValue;
    }
}

//beacon!(TestBeacon, telemetry, values(FirstTMValue, SecondTMValue, ThirdTMValue));

#[test]
fn beacon_creation() {
    let beacon = TestBeacon::new();

    let sizes = [4, (4), (2 + 4 + 4)];
    assert_eq!(beacon.bytes().len(), sizes.iter().sum());
    assert_eq!(TestBeaconDefinition::SIZES, sizes);
}

#[test]
fn beacon_insertion() {
    let mut beacon = TestBeacon::new();

    let first_value = 1234;
    let second_value = TestValue { val: 3 };
    let third_value = TestVector { x: 3, y: 3.3, z: TestValue { val: 1 }};
    beacon.insert(TestBeaconDefinition::FirstTMValue(first_value));
    beacon.insert(TestBeaconDefinition::SecondTMValue(second_value));
    beacon.insert(TestBeaconDefinition::ThirdTMValue(third_value));
    
    assert_eq!(&beacon.bytes()[0..4], first_value.to_le_bytes());
    assert_eq!(&beacon.bytes()[4..8], second_value.val.to_le_bytes());
    assert_eq!(&beacon.bytes()[8..10], third_value.x.to_le_bytes());
    assert_eq!(&beacon.bytes()[10..14], third_value.y.to_le_bytes());
    assert_eq!(&beacon.bytes()[14..18], third_value.z.val.to_le_bytes());
}

// #[test]
// fn beacon_insertion_can() {
//     let mut can_beacon = TestBeacon::new();
//     let mut beacon = TestBeacon::new();
// 
//     let first_value = 1234;
//     let second_value = TestValue { val: 3 };
//     let third_value = TestVector { x: 3, y: 3.3, z: TestValue { val: 1 }};
// 
//     can_beacon.insert(TestBeaconDefinition::from_can_topic(12, first_value.to_bytes()));
//     can_beacon.insert(TestBeaconDefinition::from_can_topic(1, first_value.to_bytes()));
//     can_beacon.insert(TestBeaconDefinition::from_can_topic(3, first_value.to_bytes()));
// 
//     beacon.insert(TestBeaconDefinition::FirstTMValue(first_value));
//     beacon.insert(TestBeaconDefinition::SecondTMValue(second_value));
//     beacon.insert(TestBeaconDefinition::ThirdTMValue(third_value));
// 
//     assert_eq!(can_beacon.bytes(), beacon.bytes());
// }
