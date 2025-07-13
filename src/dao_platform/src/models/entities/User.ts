import { User as UserDto } from "declarations/dao_association/dao_association.did.d.js";
import { candidToEnum } from "@/utils/enums";

export enum Role {
    Member = 'Member',
    Board = 'Board'
}

export class User {
    id: string;
    role: Role;

    constructor(id: string, role: Role) {
        this.id = id;
        this.role = role;
    }

    static fromDto(dto: UserDto): User {
        return new User(dto.id, candidToEnum(dto.role, Role));
    }

    toDto(): UserDto {
        return { id: this.id, role: { [this.role]: null } as UserDto['role'] };
    }
}