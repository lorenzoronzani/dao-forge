import { Principal } from "@dfinity/principal";

export const ICP = {
    NETWORK: process.env.DFX_NETWORK || ''
}

export const ICP_CANISTER_ID = {
    INTERNET_IDENTITY: Principal.fromText(process.env.CANISTER_ID_INTERNET_IDENTITY!),
    DAO_AGENCY: Principal.fromText(process.env.CANISTER_ID_DAO_AGENCY!),
    DAO_DISCOVERY: Principal.fromText(process.env.CANISTER_ID_DAO_DISCOVERY!),
    DOCUMENTS_STORAGE: Principal.fromText(process.env.CANISTER_ID_DOCUMENTS_STORAGE!),
    VOTING: Principal.fromText(process.env.CANISTER_ID_VOTING!),
    DAO_SOGC_PUBLICATION: Principal.fromText(process.env.CANISTER_ID_DAO_SOGC_PUBLICATION!),
};