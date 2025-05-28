import { ActorSubclass, Identity } from "@dfinity/agent";
import { _SERVICE } from "declarations/voting/voting.did.d.js";
import { Principal } from "@dfinity/principal";
import { createActor } from "declarations/voting";
import { VotingArgs } from "declarations/voting/voting.did.d.js";
import { Voting as VotingDto } from "declarations/voting/voting.did.d.js";
import { Voting } from "@/models/entities/Voting.js";

export class VotingService {
    private _actor: ActorSubclass<_SERVICE>;

    constructor(canisterId: Principal, identity: Identity) {
        this._actor = createActor(canisterId, {
            agentOptions: {
                identity
            }
        });
    }

    async createVoting(votingArgs: VotingArgs): Promise<Voting> {
        const votingDto = await this._actor.create_voting(votingArgs) as VotingDto;

        return Voting.fromDto(votingDto);
    }

    async getVoting(id: number): Promise<Voting> {
        const votingDto = await this._actor.get_voting(id) as VotingDto;

        return Voting.fromDto(votingDto);
    }

    async vote(votingId: number, option: string): Promise<void> {
        await this._actor.vote(votingId, option);
    }
}

