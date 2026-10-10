enum PublicationFrequency {
    Weekly,
    SemiMonthly { days: (u8, u8), annual_special: bool },
}

fn one_up_competitor(competitor_frequency: PublicationFrequency)
                     -> PublicationFrequency {
    match competitor_frequency {
        PublicationFrequency::Weekly => PublicationFrequency::SemiMonthly {
            days: (1, 15), annual_special: false
        },
        c @ PublicationFrequency::SemiMonthly{ .. } =>
            PublicationFrequency::SemiMonthly {
                annual_special: true, ..c
        }
    }
}
