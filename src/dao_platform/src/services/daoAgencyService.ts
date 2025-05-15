import { ActorSubclass, Identity } from "@dfinity/agent";
import { createActor } from "../../../declarations/dao_agency";
import { _SERVICE } from "../../../declarations/dao_agency/dao_agency.did.d.js";
import { Principal } from "@dfinity/principal";
import { DaoAssociationInitArgs } from '../../../declarations/dao_agency/dao_agency.did.js'

export class DaoAgencyService {
    private _actor: ActorSubclass<_SERVICE>;

    constructor(canisterId: Principal, identity: Identity) {
        this._actor = createActor(canisterId, {
            agentOptions: {
                identity
            }
        });
    }

    async createDaoAssociation(params: DaoAssociationInitArgs): Promise<Principal> {
        const result = await this._actor.create_dao_association(params);

        if ('Err' in result) {
            throw new Error(result.Err);
        }

        return result.Ok;
    }
}