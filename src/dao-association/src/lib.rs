use common::{Dao, LegalForm};

#[ic_cdk::query]
fn greet(name: String) -> String {
    let dao: Dao = Dao::new(name, vec!["Hello".to_string()], LegalForm::Corporation, ic_cdk::api::time());

    println!("{:?}", dao);

    format!("Hello, {:?}!", dao)
}