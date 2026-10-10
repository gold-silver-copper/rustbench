fn main() {
    enum Terminator {
        HastaLaVistaBaby,
        TalkToMyHand,
    }

    let x = Terminator::HastaLaVistaBaby;

    match x {
        Terminator::TalkToMyHand => {}
    }
}
