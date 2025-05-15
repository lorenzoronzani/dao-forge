import { ICP, ICP_CANISTER_ID } from "@/constants/icp";

export const getIdentityProvider = () => {
    if (ICP.NETWORK === "local") {
        return `http://${ICP_CANISTER_ID.INTERNET_IDENTITY}.localhost:4943`;
    } else if (ICP.NETWORK === "ic") {
        return `https://${ICP_CANISTER_ID.INTERNET_IDENTITY}.ic0.app`;
    } else {
        return `https://${ICP_CANISTER_ID.INTERNET_IDENTITY}.dfinity.network`;
    }
};