fn main() {
    // Datasets as seperate vectors
    let commissioners = [
    "Aigbogun Alamba Daudu",
    "Murtala Afee Bendu",
    "Okoracha Calstus Ogbonna",
    "Adewale Jimoh Akanbi",
    "Osauwa Faith Etiye",
    ];

    let ministries = [
    "Internal Affairs",
    "Justics",
    "Defense",
    "Power & Steel",
    "Petroleum",
    ];

    let geopolitical_zones = [
    "South West",
    "North East",
    "South South",
    "South West",
    "South East",
    ];

    // Header for the table
    println!("{:<5} {:<30} {:<20} {:<15}", "S/N", "Commissioner", "Ministry", "Geo Zone");
    println!("{:-<5} {:-<30} {:-<20} {:-<15}", "-", "-", "-", "-");

    for (serial_number, (commissioner, (ministry, zone))) in commissioners
    .iter()
    .zip(ministries.iter().zip(geopolitical_zones.iter())).enumerate()

    {
        println!("{:<5} {:<30} {:<20} {:<15}",
            serial_number + 1,
            commissioner,
            ministry,
            zone
            );
    }


}