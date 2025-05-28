import { VotingState } from "@/models/entities/Voting";

interface VotationStatusBadgeProps {
    status: VotingState;
}

export const VotationStatusBadge = ({ status }: VotationStatusBadgeProps) => {
    const getStatusColor = (status: VotingState) => {
        switch (status) {
            case VotingState.Open:
                return "bg-blue-100 text-blue-800";
            case VotingState.Closed:
                return "bg-gray-100 text-gray-800";
            default:
                return "bg-gray-100 text-gray-800";
        }
    };

    return (
        <span className={`px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(status)}`}>
            {status}
        </span>
    );
}