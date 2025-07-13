import { ActorSubclass } from "@dfinity/agent";
import { createActor } from "../../../declarations/network_call";
import { _SERVICE } from "../../../declarations/network_call/network_call.did.d.js";
import { Principal } from "@dfinity/principal";
import { CanisterAnalyzerService, MethodSignature } from "./canisterAnalyzerService";

export class NetworkCallService {
    private _actor: ActorSubclass<_SERVICE>;

    constructor(canisterId: Principal) {
        this._actor = createActor(canisterId);
    }

    getMethods(): MethodSignature[] {
        return CanisterAnalyzerService.extractMethods(this._actor);
    }
}
