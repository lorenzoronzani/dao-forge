import { Principal } from "@dfinity/principal";
import { SogcPubblication } from "@/models/entities/SogcPubblication";
import { Document } from "@/models/entities/Document";

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
    sogcPubblications: SogcPubblication[];
    board: Principal[];
    members: Principal[];
    createdAt: Date;
    documents: Document[];

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
        sogcPubblications: SogcPubblication[],
        board: Principal[],
        members: Principal[],
        createdAt: Date,
        documents: Document[]
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
        this.sogcPubblications = sogcPubblications;
        this.board = board;
        this.members = members;
        this.createdAt = createdAt;
        this.documents = documents;
    }
}