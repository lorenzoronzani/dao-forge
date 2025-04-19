import { ActorSubclass } from "@dfinity/agent";
import { createActor } from "../../../declarations/dao_association";
import { _SERVICE } from "../../../declarations/dao_association/dao_association.did.d.js";
import { DaoAssociation } from "../../../declarations/dao_association/dao_association.did.d.js";
import { Principal } from "@dfinity/principal";

export class DaoAssociationService {
    private _actor: ActorSubclass<_SERVICE>;

    constructor(canisterId: Principal) {
        this._actor = createActor(canisterId);
    }

    async getData(): Promise<DaoAssociation> {
        return this._actor.get_information();
    }
}