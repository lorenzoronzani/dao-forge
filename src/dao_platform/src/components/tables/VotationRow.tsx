import { Vote, Eye } from "lucide-react";
import { TableRow, TableCell } from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { Voting } from "@/models/entities/Voting";
import { formatDate, getTimeRemaining } from "@/utils/date";
import { VotationStatusBadge } from "../badge/VotationStatusBadge";

interface VotationRowProps {
    pool: Voting;
    canUserVote: boolean;
    openVotingModal: (pool: Voting) => void;
}

export const VotationRow = ({ pool, canUserVote, openVotingModal }: VotationRowProps) => {
    const totalVotes = Array.from(pool.result.values()).reduce((sum, count) => sum + count, 0);
    const participationRate = pool.votersWhitelist.length > 0
        ? ((totalVotes / pool.votersWhitelist.length) * 100).toFixed(0)
        : '0';

    return (
        <TableRow key={pool.id}>
            <TableCell className="font-medium">{pool.title}</TableCell>
            <TableCell>
                <VotationStatusBadge status={pool.state} />
            </TableCell>
            <TableCell>
                <div className="flex flex-col">
                    <span>{formatDate(pool.endAt.getTime())}</span>
                    <span className="text-xs text-slate-500">
                        {getTimeRemaining(pool.endAt)}
                    </span>
                </div>
            </TableCell>
            <TableCell>{participationRate}%</TableCell>
            <TableCell>
                <div className="flex items-center gap-2">
                    <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => openVotingModal(pool)}
                    >
                        {canUserVote ? (
                            <Vote className="h-4 w-4 mr-1" />
                        ) : (
                            <Eye className="h-4 w-4" />
                        )}
                    </Button>
                </div>
            </TableCell>
        </TableRow>
    )
}

export default VotationRow;
