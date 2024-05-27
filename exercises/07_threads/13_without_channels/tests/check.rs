use std::borrow::BorrowMut;
use std::sync::{Arc, RwLock};
use std::thread::spawn;

use ticket_fields::test_helpers::{ticket_description, ticket_title};
use without_channels::data::TicketDraft;
use without_channels::store::TicketStore;

#[test]
fn works() {
    let store = Arc::new(RwLock::new(TicketStore::new()));

    let mut store1 = store.clone();
    let client1 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        store1.clone().write().unwrap().add_ticket(draft)
    });

    let mut store2 = store.clone();
    let client2 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        store2.clone().write().unwrap().add_ticket(draft)
    });

    let ticket_id1 = client1.join().unwrap();
    let ticket_id2 = client2.join().unwrap();

    let ticket1 = store.read().unwrap().get(ticket_id1).unwrap();
    assert_eq!(ticket_id1, ticket1.read().unwrap().id);

    let ticket2 = store.read().unwrap().get(ticket_id2).unwrap();
    assert_eq!(ticket_id2, ticket2.read().unwrap().id);
}
