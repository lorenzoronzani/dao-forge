import { ActorSubclass } from "@dfinity/agent";
import { createActor } from "declarations/dao_association";
import { _SERVICE } from "declarations/dao_association/dao_association.did.d.js";
import { DaoAssociation as DaoAssociationDto } from "declarations/dao_association/dao_association.did.d.js";
import { Principal } from "@dfinity/principal";
import { Identity } from "@dfinity/agent";
import { CanisterAnalyzerService, MethodSignature } from "./canisterAnalyzerService";

export class DaoAssociationService {
    private _actor: ActorSubclass<_SERVICE>;

    constructor(canisterId: Principal, identity: Identity) {
        this._actor = createActor(canisterId, {
            agentOptions: {
                identity
            }
        });
    }

    async addPool(poolId: number): Promise<DaoAssociationDto> {
        return this._actor.add_pool(poolId);
    }

    async getData(): Promise<DaoAssociationDto> {
        return this._actor.get_information();
    }

    async addDocument(documentId: number): Promise<DaoAssociationDto> {
        return this._actor.add_document(documentId);
    }

    getMethods(): MethodSignature[] {
        return CanisterAnalyzerService.extractMethods(this._actor);
    }
}