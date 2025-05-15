import { ActorSubclass, Identity } from "@dfinity/agent";
import { _SERVICE, DocumentArgs, Document as DocumentDto } from "../../../declarations/documents_storage/documents_storage.did.d.js";
import { Principal } from "@dfinity/principal";
import { createActor } from "../../../declarations/documents_storage";
import { Document } from "@/models/entities/Document";

export class DocumentsStorageService {
    private _actor: ActorSubclass<_SERVICE>;

    constructor(canisterId: Principal, identity: Identity) {
        this._actor = createActor(canisterId, {
            agentOptions: {
                identity
            }
        });
    }

    async uploadDocument(params: DocumentArgs): Promise<Document> {
        const documentDto = await this._actor.store_document(params) as DocumentDto;
        return Document.fromDto(documentDto);
    }

    async getDocument(id: number): Promise<Document> {
        const documentDto = await this._actor.get_document(id) as DocumentDto;
        return Document.fromDto(documentDto);
    }
}

