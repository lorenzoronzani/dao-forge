import { Principal } from "@dfinity/principal";
import { Dao, LegalForm, OrganizationStatus } from "@/models/entities/Dao";
import { SogcPublication } from "@/models/entities/SogcPublication";
import { DaoAssociation as DaoAssociationDto } from "declarations/dao_association/dao_association.did.d.js";
import { candidToEnum } from "@/utils/enums";
import { Document } from "@/models/entities/Document";
import { Voting } from "./Voting";
import { User } from "./User";

export class DaoAssociation extends Dao {
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
        super(
            principal,
            name,
            address,
            zip,
            town,
            legalForm,
            status,
            uid,
            chId,
            frcId,
            purpose,
            sogcPublications,
            members,
            createdAt,
            documents,
            pools
        );
    }

    static fromDto(dto: DaoAssociationDto, principal: Principal, documents: Document[], pools: Voting[], sogcPublications: SogcPublication[]): DaoAssociation {
        return new DaoAssociation(
            principal,
            dto.name,
            dto.address,
            dto.zip,
            dto.town,
            candidToEnum(dto.legal_form, LegalForm),
            candidToEnum(dto.status, OrganizationStatus),
            dto.uid,
            dto.ch_id,
            dto.frc_id,
            dto.purpose,
            sogcPublications,
            dto.members.map(p => User.fromDto(p)),
            new Date(Number(dto.created_at)),
            documents,
            pools
        );
    }
}