#![cfg(test)]

use crate::*;

use std::env;

#[test]
fn it_works() {
    println!("{:?}", env::var("QT_QPA_PLATFORM_PLUGIN_PATH"));

    let mut origin = Identity::default();
    origin.id = 10;
    origin.product_name = Str32::new("helloss");
    origin.supported_groups = (DF::APP2 as u32) | (DG::IMAGE as u32) | (DG::CONTROL as u32);
    // origin.product_family = Str32::new("qsdqsdsqd");

    //    let mut dest = Identity::default();
    // entry_point.allocate(100);
    //    println!("alloc {:?}", entry_point.mem_allocate);

    let mut client = Client::new().unwrap();

    // let c = ['9'; 100];

    let open_dsm = client.open_dsm(origin.clone()).unwrap();

    // let entry_point = open_dsm.get_entrypoint();

    let selected_identity = open_dsm.open_select_ds();
    println!("selected_identity {:?}", selected_identity);

    let open_ds = open_dsm.open_ds(selected_identity.unwrap()).unwrap();

    let enabled_ds = open_ds
        .user_interface_enable_ds(&UserInterface::new(true, true))
        .unwrap()
        .unwrap();

    let ev = enabled_ds.recv().unwrap();
    println!("ev {:?}", ev);
    match ev {
        DSEvent::Ready(event) => {
            let ready_ds = enabled_ds.ready(event);
            println!("info {:?}", ready_ds.image_info());
            ready_ds.image_native_transfer().unwrap();
        }
        _ => {
            println!("some other event");
        }
    }
}
