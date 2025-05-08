import { Principal } from "@dfinity/principal";
import { Dao, LegalForm, OrganizationStatus } from "@/models/entities/Dao";
import { SogcPubblication } from "@/models/entities/SogcPubblication";
import { DaoAssociation as DaoAssociationDto } from "declarations/dao_association/dao_association.did.d.js";
import { candidToEnum } from "@/utils/enums";

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
        sogcPubblications: SogcPubblication[],
        board: Principal[],
        members: Principal[],
        createdAt: Date
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
            sogcPubblications,
            board,
            members,
            createdAt
        );
    }

    static fromDto(dto: DaoAssociationDto, principal: Principal): DaoAssociation {
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
            [],
            dto.board.map(p => Principal.fromText(p)),
            dto.members.map(p => Principal.fromText(p)),
            new Date(Number(dto.created_at))
        );
    }
}