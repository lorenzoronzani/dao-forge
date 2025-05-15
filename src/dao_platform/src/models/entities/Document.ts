import { Principal } from "@dfinity/principal";
import { Document as DocumentDto } from "declarations/documents_storage/documents_storage.did.d.js";

export class Document {
    id: number;
    name: string;
    owner: Principal;
    content: Uint8Array;
    contentType: string;
    updatedAt: Date;

    constructor(
        id: number,
        name: string,
        owner: Principal,
        content: Uint8Array,
        contentType: string,
        updatedAt: Date
    ) {
        this.id = id;
        this.name = name;
        this.owner = owner;
        this.content = content;
        this.contentType = contentType;
        this.updatedAt = updatedAt;
    }

    static fromDto(dto: DocumentDto): Document {
        return new Document(
            dto.id,
            dto.name,
            dto.owner,
            Uint8Array.from(dto.content),
            dto.content_type,
            new Date(Number(dto.updated_at))
        );
    }
}