use crate::contract::setup;
use fuels::core::types::SizedAsciiString;

mod success {
    use super::*;

    #[tokio::test]
    async fn should_intialize_contract_name_symbol() {
        let (instance, _id) = setup::get_contract_instance().await;
        let n = string_to_ascii(String::from("Eth"));
        let s = string_to_ascii(String::from("ETH"));

        let result = instance.methods().constructor(n, s).call().await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn should_read_contract_name_symbol() {        
        let (instance, _id) = setup::get_contract_instance().await;
        let n = string_to_ascii(String::from("Eth"));
        let s = string_to_ascii(String::from("ETH"));

        let _ = instance.methods().constructor(n, s).call().await;

        let name = instance.methods().name().call().await.unwrap();
        let symbol = instance.methods().symbol().call().await.unwrap();

        assert_eq!(name.value, "Eth");
        assert_eq!(symbol.value, "ETH");
    }
}

pub fn string_to_ascii(name: String) -> SizedAsciiString<3> {
    match SizedAsciiString::<3>::new(name) {
        Err(_) => panic!("error converting"),
        Ok(v) => v
    }
}
