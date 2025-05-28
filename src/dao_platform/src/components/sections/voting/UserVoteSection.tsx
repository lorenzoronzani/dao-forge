import { CheckCircle } from "lucide-react";

interface UserVoteSectionProps {
    userVote: string;
}

export const UserVoteSection = ({ userVote }: UserVoteSectionProps) => {
    return (
        <div className="bg-green-50 border border-green-200 rounded-lg p-3">
            <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-600" />
                <span className="text-green-800 font-medium">You voted for: {userVote}</span>
            </div>
        </div>
    );
};

export default UserVoteSection;