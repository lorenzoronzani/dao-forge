import { ActorSubclass } from "@dfinity/agent";
import { createActor } from "../../../declarations/dao_discovery";
import { Principal } from "@dfinity/principal";
import { _SERVICE } from "../../../declarations/dao_discovery/dao_discovery.did.d.js";
import { DaoAssociationService } from "./daoAssocationService";
import { Dao } from "@/models/entities/Dao";
import { DaoAssociation } from "@/models/entities/DaoAssociation";

export class DaoDiscoveryService {
    private _actor: ActorSubclass<_SERVICE>;

    constructor() {
        this._actor = createActor(import.meta.env.VITE_CANISTER_ID_DAO_DISCOVERY);
    }

    async getRandomDaos(count: number = 6): Promise<Dao[]> {
        const daosPrincipals = await this._actor.get_random_daos([count]);

        const daos = await Promise.all(daosPrincipals.map(principal => new DaoAssociationService(principal).getData()));

        return daos.map(dao => DaoAssociation.fromDto(dao));
    }

    async getUserDaos(user: Principal): Promise<Dao[]> {
        const daosPrincipals = await this._actor.get_user_daos([user]);

        const daos = await Promise.all(daosPrincipals.map(principal => new DaoAssociationService(principal).getData()));

        return daos.map(dao => DaoAssociation.fromDto(dao));
    }
}