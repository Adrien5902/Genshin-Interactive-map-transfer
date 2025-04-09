use crate::{off_chests_to_uno, read_off_chests, read_uno_chests};

const REF_VALUES: &[(u64, u64)] = &[
    (62, 319),
    (50088, 52509),
    (50078, 52519),
    (4732, 6822),
    (2170, 4977),
    (6468, 9919),
    (68369, 69131),
    (72309, 73223),
    (72321, 73211),
    (72332, 73200),
    (13493, 16316),
    (19259, 21966),
];

#[test]
fn test() {
    let off_chests = read_off_chests().unwrap();
    let uno_chests = read_uno_chests().unwrap();

    let off_uno_chests_map = off_chests_to_uno(&off_chests, &uno_chests).unwrap();

    let results = off_uno_chests_map.iter().filter_map(|(off, uno)| {
        REF_VALUES
            .iter()
            .find(|ref_value| ref_value.0 == off.id)
            .map(|ref_value| {
                (
                    ref_value.1 == uno.unwrap().id,
                    off,
                    uno.unwrap(),
                    ref_value.1,
                )
            })
    });

    for result in results {
        assert!(
            result.0,
            "found uno : {} for {} expected {} (type: {})\nexpected: https://genshin-impact-map.appsample.com/?type=o{}&id={}&mid=2\nfound: https://genshin-impact-map.appsample.com/?type=o{}&id={}&mid=2",
            result.2.id, result.1.id, result.3, result.1.label_id, result.1.label_id, result.3, result.1.label_id, result.2.id
        )
    }
}
