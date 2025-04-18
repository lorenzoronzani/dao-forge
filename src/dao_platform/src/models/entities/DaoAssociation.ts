import { Principal } from "@dfinity/principal";
import { Dao, LegalForm, OrganizationStatus } from "./Dao";
import { SogcPubblication } from "./SogcPubblication";

export class DaoAssociation extends Dao {
    constructor(
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
}