import { ActorSubclass } from "@dfinity/agent";
import { createActor } from "../../../declarations/dao_association";
import { _SERVICE } from "../../../declarations/dao_association/dao_association.did.d.js";
import { DaoAssociation } from "../../../declarations/dao_association/dao_association.did.d.js";
import { Principal } from "@dfinity/principal";
import { Identity } from "@dfinity/agent";

export class DaoAssociationService {
    private _actor: ActorSubclass<_SERVICE>;

    constructor(canisterId: Principal, identity: Identity) {
        this._actor = createActor(canisterId, {
            agentOptions: {
                identity
            }
        });
    }

    async getData(): Promise<DaoAssociation> {
        return this._actor.get_information();
    }
}