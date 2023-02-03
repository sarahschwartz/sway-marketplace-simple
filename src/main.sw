contract;

use std::{
    auth::msg_sender,
    context::msg_amount,
    constants::BASE_ASSET_ID,
    call_frames::msg_asset_id,
    token::transfer
};

struct Item {
   id: u64,
   price: u64,
   owner: Identity,
   metadata: str[20],
   total_bought: u64
}

storage {
    item_count: u64 = 0,
    item_map: StorageMap<u64, Item> = StorageMap{},
}

abi Marketplace {
    #[storage(read, write)]
    fn list_item(price: u64, metadata: str[20]);

    #[storage(read, write), payable]
    fn buy_item(id: u64);

    #[storage(read)]
    fn get_count() -> u64;

    #[storage(read)]
    fn get_item(id: u64) -> Item;
}

impl Marketplace for Contract {
    #[storage(read, write)]
    fn list_item(price: u64, metadata: str[20]){
        storage.item_count += 1;

        let sender = msg_sender().unwrap();

        let new_item = Item {
            id: storage.item_count,
            price: price,
            owner: sender,
            metadata: metadata,
            total_bought: 0
        };

        storage.item_map.insert(storage.item_count, new_item);
    }

    #[storage(read, write), payable]
    fn buy_item(id: u64){
        let sender = msg_sender().unwrap();

        let asset_id = msg_asset_id();
        require(asset_id == BASE_ASSET_ID, "wrong assset");

        let amount = msg_amount();
        let mut item = storage.item_map.get(id);
        require(amount >= item.price, "wrong amount");

        item.total_bought += 1;
        storage.item_map.insert(id, item);

        transfer(amount, asset_id, item.owner);
    }

    #[storage(read)]
    fn get_count() -> u64{
        storage.item_count
    }

    #[storage(read)]
    fn get_item(id: u64) -> Item {
        storage.item_map.get(id)
    }
}
