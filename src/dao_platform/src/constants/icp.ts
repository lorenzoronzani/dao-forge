export const ICP = {
    NETWORK: process.env.DFX_NETWORK || ''
}

export const ICP_CANISTER_ID = {
    INTERNET_IDENTITY: process.env.CANISTER_ID_INTERNET_IDENTITY || '',
    DAO_AGENCY: process.env.CANISTER_ID_DAO_AGENCY || '',
    DAO_DISCOVERY: process.env.CANISTER_ID_DAO_DISCOVERY || '',
    DOCUMENTS_STORAGE: process.env.CANISTER_ID_DOCUMENTS_STORAGE || '',
};