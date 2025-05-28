import { ActorSubclass } from "@dfinity/agent";
import { createActor } from "../../../declarations/dao_discovery";
import { Principal } from "@dfinity/principal";
import { _SERVICE } from "../../../declarations/dao_discovery/dao_discovery.did.d.js";
import { DaoAssociationService } from "./daoAssocationService";
import { Dao } from "@/models/entities/Dao";
import { DaoAssociation } from "@/models/entities/DaoAssociation";
import { ICP_CANISTER_ID } from "@/constants/icp";
import { Identity } from "@dfinity/agent";
import { DocumentsStorageService } from "./documentsStorageService";
import { VotingService } from "./votingService";

export class DaoDiscoveryService {
    private _actor: ActorSubclass<_SERVICE>;
    private _identity: Identity;
    private _documentsStorageService: DocumentsStorageService;
    private _votingService: VotingService;

    constructor(identity: Identity) {
        this._identity = identity;
        this._actor = createActor(ICP_CANISTER_ID.DAO_DISCOVERY, {
            agentOptions: {
                identity
            }
        });
        this._documentsStorageService = new DocumentsStorageService(ICP_CANISTER_ID.DOCUMENTS_STORAGE, identity);
        this._votingService = new VotingService(ICP_CANISTER_ID.VOTING, identity);
    }

    async getDaos(daosPrincipals: Principal[]): Promise<Dao[]> {
        const daos = await Promise.all(daosPrincipals.map(async principal => {
            const dto = await new DaoAssociationService(principal, this._identity).getData()

            const documents = await Promise.all(Array.from(dto.documents).map(async documentId => await this._documentsStorageService.getDocument(documentId)));

            const pools = await Promise.all(Array.from(dto.pools).map(async poolId => await this._votingService.getVoting(poolId)));

            return DaoAssociation.fromDto(dto, principal, documents, pools);
        }));

        return daos;
    }

    async getRandomDaos(count: number = 6): Promise<Dao[]> {
        const daosPrincipals = await this._actor.get_random_daos([count]);

        return this.getDaos(daosPrincipals);
    }

    async getUserDaos(user?: Principal): Promise<Dao[]> {
        const daosPrincipals = await this._actor.get_user_daos(user ? [user] : []);

        return this.getDaos(daosPrincipals);
    }
}