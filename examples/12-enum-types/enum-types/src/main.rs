#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    BarossaValley,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: &WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn how_popular(w: &WineRegions) -> String {
    match w {
        WineRegions::BarossaValley => "I like it!".to_string(),
        WineRegions::Bordeaux => "Classic!".to_string(),
        WineRegions::Tuscany => "Toss a coin to the Witcher!".to_string(),
        WineRegions::Rioja => "Cheap and cheerful!".to_string(),
        _ => "Not sure about this one.".to_string(),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Penfolds Grange"),
        region: WineRegions::BarossaValley,
    };

    // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
    supported_regions(&wine1.region);
    supported_regions(&WineRegions::Rioja);
    println!("How popular is {}?", how_popular(&wine1.region));
    println!("How popular is {}?", how_popular(&WineRegions::Rioja));
    println!("How popular is {}?", how_popular(&wine3.region));
}
