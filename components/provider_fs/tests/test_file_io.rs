// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::erased::*;
use icu_provider::prelude::*;
use icu_provider::structs;
use icu_provider_fs::FsDataProvider;
use std::borrow::Cow;

const EXPECTED_RU_DATA: structs::plurals::PluralRuleStringsV1 =
    structs::plurals::PluralRuleStringsV1 {
        zero: None,
        one: Some(Cow::Borrowed("v = 0 and i % 10 = 1 and i % 100 != 11")),
        two: None,
        few: Some(Cow::Borrowed(
            "v = 0 and i % 10 = 2..4 and i % 100 != 12..14",
        )),
        many: Some(Cow::Borrowed(
            "v = 0 and i % 10 = 0 or v = 0 and i % 10 = 5..9 or v = 0 and i % 100 = 11..14",
        )),
    };

#[cfg(feature = "bincode")]
const EXPECTED_SR_DATA: structs::plurals::PluralRuleStringsV1 =
    structs::plurals::PluralRuleStringsV1 {
        zero: None,
        one: Some(Cow::Borrowed(
            "v = 0 and i % 10 = 1 and i % 100 != 11 or f % 10 = 1 and f % 100 != 11",
        )),
        two: None,
        few: Some(Cow::Borrowed(
            "v = 0 and i % 10 = 2..4 and i % 100 != 12..14 or f % 10 = 2..4 and f % 100 != 12..14",
        )),
        many: None,
    };

fn get_request(langid: LanguageIdentifier) -> DataRequest {
    DataRequest {
        resource_path: ResourcePath {
            key: structs::plurals::key::CARDINAL_V1,
            options: ResourceOptions {
                variant: None,
                langid: Some(langid),
            },
        },
    }
}

#[test]
fn test_json() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    let plurals_data: Cow<structs::plurals::PluralRuleStringsV1> = provider
        .load_payload(&get_request(langid!("ru")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(*plurals_data, EXPECTED_RU_DATA);
}

#[test]
fn test_json_dyn_erased() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    let plurals_data: Cow<structs::plurals::PluralRuleStringsV1> = (&provider
        as &dyn ErasedDataProvider)
        .load_payload(&get_request(langid!("ru")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(*plurals_data, EXPECTED_RU_DATA);
}

#[test]
fn test_json_errors() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");
    let mut receiver = DataReceiver::<structs::plurals::PluralRuleStringsV1>::new();

    assert!(matches!(
        provider.load_to_receiver(
            &DataRequest {
                resource_path: ResourcePath {
                    key: structs::plurals::key::CARDINAL_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("ru"))
                    }
                }
            },
            &mut receiver
        ),
        Ok(_)
    ));

    receiver.reset();

    assert!(matches!(
        provider.load_to_receiver(
            &DataRequest {
                resource_path: ResourcePath {
                    key: structs::plurals::key::CARDINAL_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("zh"))
                    }
                }
            },
            &mut receiver
        ),
        Err(DataError::UnavailableResourceOptions(_))
    ));

    receiver.reset();

    assert!(matches!(
        provider.load_to_receiver(
            &DataRequest {
                resource_path: ResourcePath {
                    key: structs::plurals::key::ORDINAL_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("ru"))
                    }
                }
            },
            &mut receiver
        ),
        Err(DataError::UnsupportedResourceKey(_))
    ));

    receiver.reset();

    assert!(matches!(
        provider.load_to_receiver(
            &DataRequest {
                resource_path: ResourcePath {
                    key: structs::dates::key::GREGORY_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("ru"))
                    }
                }
            },
            &mut receiver
        ),
        Err(DataError::UnsupportedCategory(_))
    ));
}

#[test]
#[cfg(feature = "bincode")]
fn test_bincode() {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    let plurals_data: Cow<structs::plurals::PluralRuleStringsV1> = provider
        .load_payload(&get_request(langid!("sr")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(*plurals_data, EXPECTED_SR_DATA);
}

#[test]
#[cfg(feature = "bincode")]
fn test_bincode_dyn_erased() {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    let plurals_data: Cow<structs::plurals::PluralRuleStringsV1> = (&provider
        as &dyn ErasedDataProvider)
        .load_payload(&get_request(langid!("sr")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(*plurals_data, EXPECTED_SR_DATA);
}
