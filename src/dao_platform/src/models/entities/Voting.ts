import { Principal } from "@dfinity/principal";
import { Action } from "./Action";
import { Voting as VotingDto } from "declarations/voting/voting.did.d.js";
import { candidToEnum } from "@/utils/enums";
import { Notification } from "./Notification";

export enum VotingState {
    Open = 'Open',
    Closed = 'Closed'
}

export class Voting {
    id: number;
    title: string;
    description: string;
    options: Map<string, Action | null>;
    result: Map<string, number>;
    owner: Principal;
    createdAt: Date;
    endAt: Date;
    state: VotingState;
    daoId: Principal;
    updatedAt: Date;
    approvalThreshold: number;
    quorum: number;
    votersWhitelist: Principal[];
    votersCast: Map<string, string>;
    notification: Notification | null;

    constructor(
        id: number,
        title: string,
        description: string,
        options: Map<string, Action | null>,
        result: Map<string, number>,
        owner: Principal,
        createdAt: Date,
        endAt: Date,
        state: VotingState,
        daoId: Principal,
        updatedAt: Date,
        approvalThreshold: number,
        quorum: number,
        votersWhitelist: Principal[],
        votersCast: Map<string, string>,
        notification: Notification | null,
    ) {
        this.id = id;
        this.title = title;
        this.description = description;
        this.options = options;
        this.result = result;
        this.owner = owner;
        this.createdAt = createdAt;
        this.endAt = endAt;
        this.state = state;
        this.daoId = daoId;
        this.updatedAt = updatedAt;
        this.approvalThreshold = approvalThreshold;
        this.quorum = quorum;
        this.votersWhitelist = votersWhitelist;
        this.votersCast = votersCast;
        this.notification = notification;
    }

    static fromDto(dto: VotingDto): Voting {
        return new Voting(
            dto.id,
            dto.title,
            dto.description,
            new Map(dto.options.map(([key, value]) => [key, value[0] ? Action.fromDto(value[0]) : null])),
            new Map(dto.result),
            dto.owner,
            new Date(Number(dto.created_at)),
            new Date(Number(dto.end_at)),
            candidToEnum(dto.state, VotingState),
            dto.dao_id,
            new Date(Number(dto.updated_at)),
            dto.approval_threshold,
            dto.quorum,
            dto.voters_whitelist,
            new Map(dto.voters_cast.map(([key, value]) => [key.toText(), value])),
            dto.notification[0] ? Notification.fromDto(dto.notification[0]) : null,
        );
    }
}
