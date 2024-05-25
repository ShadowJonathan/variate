use variate_lib::Extractor;
use variate_synapse;

// const PASSWORD: &str = "CUZBABYYOUREAFIRE";
const PASSWORD: &str = "Cuzbabytonight";

fn main() {
    let c = variate_synapse::make_config("localhost", None, "synapse", PASSWORD, "synapse");

    let mut e = variate_synapse::make_extractor(c);

    // let ids = e.room_e().all_known_ids();

    // dbg!(ids);

    let rooms = e.room_e().all_known_ids();
    let in_rooms = &rooms[..10];

    for room in in_rooms {
        dbg!(room);

        let mut counter = 0;
        for event in e.pdu_e().for_room(room) {
            counter += 1;
            if counter % 1000 == 0 {
                dbg!(counter);
            }
        }

        if counter % 1000 != 0 {
            dbg!(counter);
        }
    }
}
