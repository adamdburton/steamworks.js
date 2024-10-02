use napi_derive::napi;
use steamworks;
use napi::bindgen_prelude::*;

#[napi]
pub mod inventory {
    use super::*;

    #[napi(object)]
    #[derive(Clone)]
    pub struct NapiSteamItemDetails {
        pub item_id: BigInt,
        pub definition: i32,
        pub quantity: u16,
        pub flags: u16,
    }

    impl From<steamworks::SteamItemDetails> for NapiSteamItemDetails {
        fn from(details: steamworks::SteamItemDetails) -> Self {
            NapiSteamItemDetails {
                item_id: BigInt::from(details.item_id.0),
                definition: details.definition.0,
                quantity: details.quantity,
                flags: details.flags,
            }
        }
    }

    #[napi]
    #[derive(Debug)]
    pub enum NapiInventoryError {
        OperationFailed,
        GetResultItemsFailed,
        InvalidInput,
        Timeout,
    }

    impl From<steamworks::InventoryError> for NapiInventoryError {
        fn from(error: steamworks::InventoryError) -> Self {
            match error {
                steamworks::InventoryError::OperationFailed => NapiInventoryError::OperationFailed,
                steamworks::InventoryError::GetResultItemsFailed => NapiInventoryError::GetResultItemsFailed,
                steamworks::InventoryError::InvalidInput => NapiInventoryError::InvalidInput,
                steamworks::InventoryError::Timeout => NapiInventoryError::Timeout,
            }
        }
    }

    #[napi]
    pub fn get_all_items() -> Result<Vec<NapiSteamItemDetails>> {
        let client = crate::client::get_client();
        client.inventory().get_all_items()
            .map(|items| items.into_iter().map(NapiSteamItemDetails::from).collect())
            .map_err(|e| {
                let napi_error: NapiInventoryError = e.into();
                Error::new(Status::GenericFailure, format!("{:?}", napi_error))
            })
    }

    #[napi]
    pub fn consume_item(item_id: BigInt, quantity: u32) -> Result<()> {
        let client = crate::client::get_client();
        let item_id = steamworks::SteamItemInstanceID(item_id.get_u64().1);

        client.inventory().consume_item(item_id, quantity)
            .map_err(|e| {
                let napi_error: NapiInventoryError = e.into();
                Error::new(Status::GenericFailure, format!("{:?}", napi_error))
            })
    }

    #[napi]
    pub fn start_purchase(item_id: i32, quantity: u32) {
        let client = crate::client::get_client();
        let items_to_purchase = vec![(steamworks::SteamItemDef(item_id), quantity)];

        client.inventory().start_purchase(&items_to_purchase, |result| {
            match result {
                Ok(purchase_result) => {
                    println!("Purchase started successfully!");
                    println!("Order ID: {}", purchase_result.order_id);
                    println!("Transaction ID: {}", purchase_result.trans_id);
                },
                Err(error) => {
                    println!("Failed to start purchase: {:?}", error);
                }
            }
        });
    }
}