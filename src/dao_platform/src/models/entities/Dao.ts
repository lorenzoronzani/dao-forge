import { Principal } from "@dfinity/principal";
import { SogcPublication } from "@/models/entities/SogcPublication";
import { Document } from "@/models/entities/Document";
import { Voting } from "./Voting";
import { User } from "./User";

export enum LegalForm {
    Corporation = 'Corporation',
    LimitedLiabilityCompany = 'LimitedLiabilityCompany',
    Association = 'Association'
}

export enum OrganizationStatus {
    Active = 'Active',
    Liquidation = 'Liquidation',
    Dissolved = 'Dissolved'
}

export class Dao {
    principal: Principal;
    name: string;
    address: string;
    zip: number;
    town: string;
    legalForm: LegalForm;
    status: OrganizationStatus;
    uid: string;
    chId: string;
    frcId: number;
    purpose: string;
    sogcPublications: SogcPublication[];
    members: User[];
    createdAt: Date;
    documents: Document[];
    pools: Voting[];

    constructor(
        principal: Principal,
        name: string,
        address: string,
        zip: number,
        town: string,
        legalForm: LegalForm,
        status: OrganizationStatus,
        uid: string,
        chId: string,
        frcId: number,
        purpose: string,
        sogcPublications: SogcPublication[],
        members: User[],
        createdAt: Date,
        documents: Document[],
        pools: Voting[]
    ) {
        this.principal = principal;
        this.name = name;
        this.address = address;
        this.zip = zip;
        this.town = town;
        this.legalForm = legalForm;
        this.status = status;
        this.uid = uid;
        this.chId = chId;
        this.frcId = frcId;
        this.purpose = purpose;
        this.sogcPublications = sogcPublications;
        this.members = members;
        this.createdAt = createdAt;
        this.documents = documents;
        this.pools = pools;
    }
}