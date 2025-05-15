import { Dao } from "@/models/entities/Dao";
import { MemberCard } from "../../components/cards/MemberCard";

interface MembersViewProps {
    dao: Dao;
}

export const MembersView = ({ dao }: MembersViewProps) => {
    return (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <MemberCard title="Board" description="People with administrative roles in the DAO" members={dao.board} />
            <MemberCard title="Members" description="All participating members of the DAO" members={dao.members} />
        </div>
    );
};