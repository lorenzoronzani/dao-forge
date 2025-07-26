import { ICP, ICP_CANISTER_ID } from "@/constants/icp";

export const getIdentityProvider = () => {
    if (ICP.NETWORK === "ic") {
        return `https://identity.ic0.app`;
    } else {
        return `http://${ICP_CANISTER_ID.INTERNET_IDENTITY}.localhost:4943`;
    }
};