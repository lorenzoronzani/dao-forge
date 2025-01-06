use ic_cdk::macros::init;
use ic_cdk::api::time;
use dao_association::DaoAssociation;

// struct InitArgs {
//     name: String,
//     members: Vec<String>,
// }
//
// #[init]
// fn canister_init(args: InitArgs) {
//     let dao_association = DaoAssociation::new(args.name, args.members, time());
// }