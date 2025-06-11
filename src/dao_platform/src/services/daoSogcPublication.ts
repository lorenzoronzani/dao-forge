import { ActorSubclass } from "@dfinity/agent";
import { _SERVICE, SogcPublication as SogcPublicationDto } from "../../../declarations/dao_sogc_publication/dao_sogc_publication.did.js";
import { Principal } from "@dfinity/principal";
import { Identity } from "@dfinity/agent";
import { createActor } from "../../../declarations/dao_sogc_publication";
import { SogcPublication } from "@/models/entities/SogcPublication";

export class DaoSogcPublicationService {
    private _actor: ActorSubclass<_SERVICE>;

    constructor(canisterId: Principal, identity: Identity) {
        this._actor = createActor(canisterId, {
            agentOptions: {
                identity
            }
        });
    }

    async getPublication(id: number): Promise<SogcPublication> {
        const publicationDto = await this._actor.get_sogc_publication(id) as SogcPublicationDto;

        return SogcPublication.fromDto(publicationDto);
    }
}