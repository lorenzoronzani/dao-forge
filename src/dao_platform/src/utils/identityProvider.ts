export const getIdentityProvider = () => {
    if (import.meta.env.VITE_DFX_NETWORK === "local") {
        return `http://${import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`;
    } else if (import.meta.env.VITE_DFX_NETWORK === "ic") {
        return `https://${import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY}.ic0.app`;
    } else {
        return `https://${import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY}.dfinity.network`;
    }
};