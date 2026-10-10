use task::{Meeting, total_minutes};

#[test]
fn adds_up_meeting_minutes() {
    let day = [
        Meeting {
            title: "standup".into(),
            start: 540,
            end: 555,
        },
        Meeting {
            title: "planning".into(),
            start: 780,
            end: 855,
        },
    ];
    assert_eq!(total_minutes(&day), 90);
}
