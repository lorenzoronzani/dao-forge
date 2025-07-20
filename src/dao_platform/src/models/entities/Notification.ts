import { Notification as NotificationDto } from "../../../../declarations/voting/voting.did.js";

export class Notification {
    email: String;
    pdf_bytes: number[];

    constructor(email: String, pdf_bytes: number[]) {
        this.email = email;
        this.pdf_bytes = pdf_bytes;
    }

    static fromDto(dto: NotificationDto): Notification {
        return new Notification(dto.email, Array.from(dto.pdf_bytes))
    }
}