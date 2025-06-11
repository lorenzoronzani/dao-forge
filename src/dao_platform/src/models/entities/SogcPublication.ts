import { candidToEnum } from "@/utils/enums";
import { SogcPublication as SogcPublicationDto } from "declarations/dao_sogc_publication/dao_sogc_publication.did.d.js";

export enum Mutation {
    ChangeOfAddress = 'ChangeOfAddress',
    ChangeOfStatus = 'ChangeOfStatus',
    ChangeOfCompany = 'ChangeOfCompany',
    NewInscription = 'NewInscription'
}

export class SogcPublication {
    sogcId: number;
    publicationSogcDate: Date;
    dailyNumber: number;
    publicationDate: Date;
    mutations: Mutation[];
    description: string;

    constructor(sogc_id: number, publication_sogc_date: Date, daily_number: number, publication_date: Date, mutations: Mutation[], description: string) {
        this.sogcId = sogc_id;
        this.publicationSogcDate = publication_sogc_date;
        this.dailyNumber = daily_number;
        this.publicationDate = publication_date;
        this.mutations = mutations;
        this.description = description;
    }

    static fromDto(dto: SogcPublicationDto): SogcPublication {
        return new SogcPublication(
            dto.sogc_id,
            new Date(Number(dto.publication_sogc_date)),
            dto.daily_number,
            new Date(Number(dto.publication_date)),
            dto.mutations.map((mutation) => candidToEnum(mutation, Mutation)),
            dto.description
        );
    }
}