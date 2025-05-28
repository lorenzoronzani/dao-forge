import { Principal } from "@dfinity/principal";
import { Action as ActionDto } from "../../../../declarations/voting/voting.did.js";

export class Action {
    canisterId: Principal;
    method: string;
    args: string[];

    constructor(
        canisterId: Principal,
        method: string,
        args: string[]
    ) {
        this.canisterId = canisterId;
        this.method = method;
        this.args = args;
    }

    static fromDto(dto: ActionDto): Action {
        return new Action(
            dto.canister_id,
            dto.method,
            dto.args
        );
    }

    toDto(): ActionDto {
        return {
            canister_id: this.canisterId,
            method: this.method,
            args: this.args
        }
    }
}