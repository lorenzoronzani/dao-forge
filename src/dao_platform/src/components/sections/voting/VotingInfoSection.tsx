import { getTimeRemaining } from "@/utils/date";
import { Voting } from "@/models/entities/Voting";
import { AlertCircle, Calendar, CheckCircle, Clock, Users, Vote } from "lucide-react";

interface VotingInfoSectionProps {
    voting: Voting;
}

export const VotingInfoSection = ({ voting }: VotingInfoSectionProps) => {
    const totalVotes = Array.from(voting.result.values()).reduce((sum, count) => sum + count, 0);

    return (
        <div className="grid grid-cols-2 gap-4 text-sm">
            <div className="flex items-center gap-2">
                <Calendar className="h-4 w-4 text-slate-500" />
                <span className="text-slate-500">Ends:</span>
                <span>{voting.endAt.toLocaleDateString()} {voting.endAt.toLocaleTimeString()}</span>
            </div>
            <div className="flex items-center gap-2">
                <Clock className="h-4 w-4 text-slate-500" />
                <span className="text-slate-500">Status:</span>
                <span>{getTimeRemaining(voting.endAt)}</span>
            </div>
            <div className="flex items-center gap-2">
                <Users className="h-4 w-4 text-slate-500" />
                <span className="text-slate-500">Eligible Voters:</span>
                <span>{voting.votersWhitelist.length}</span>
            </div>
            <div className="flex items-center gap-2">
                <Vote className="h-4 w-4 text-slate-500" />
                <span className="text-slate-500">Votes Cast:</span>
                <span>{totalVotes}</span>
            </div>
            <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-slate-500" />
                <span className="text-slate-500">Quorum Required:</span>
                <span>{voting.quorum}%</span>
            </div>
            <div className="flex items-center gap-2">
                <AlertCircle className="h-4 w-4 text-slate-500" />
                <span className="text-slate-500">Approval Threshold:</span>
                <span>{voting.approvalThreshold}%</span>
            </div>
        </div>
    );
};

export default VotingInfoSection;
