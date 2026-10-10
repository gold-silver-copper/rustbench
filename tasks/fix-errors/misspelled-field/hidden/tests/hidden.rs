use task::Parcel;

#[test]
fn volume() {
    let ps = [Parcel { width: 1.0, height: 2.0, depth: 3.0 }, Parcel { width: 2.0, height: 2.0, depth: 2.0 }];
    assert_eq!(task::total_volume(&ps), 14.0);
}
