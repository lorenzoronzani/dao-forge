import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Table, TableBody, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { File } from 'lucide-react';
import { useNavigate } from "react-router";
import { Dao } from "@/models/entities/Dao";
import { Voting, VotingState } from "@/models/entities/Voting";
import { useState } from "react";
import VotationRow from "@/components/tables/VotationRow";
import { useAuthentication } from "@/providers/AuthenticationProvider";
import { VotingModal } from "@/components/modals/VotingModal";
import { toast } from "@/hooks/use-toast";
import { VotingService } from "@/services/votingService";
import { ICP_CANISTER_ID } from "@/constants/icp";
import { useDao } from "@/providers/DaoProvider";


interface VotationsViewProps {
    dao: Dao,
}

export const VotationsView = ({ dao }: VotationsViewProps) => {
    const navigate = useNavigate();
    const { userPrincipal, identity } = useAuthentication();
    const { refreshData } = useDao();
    const [selectedVoting, setSelectedVoting] = useState<Voting>();
    const [isVotingModalOpen, setIsVotingModalOpen] = useState<boolean>(false);

    const canUserVote = (voting: Voting): boolean => {
        return voting.state === VotingState.Open &&
            voting.votersWhitelist.some(p => p.toText() === userPrincipal.toText()) &&
            !voting.votersCast.has(userPrincipal.toText());
    };

    const openVotingModal = (voting: Voting) => {
        setSelectedVoting(voting);
        setIsVotingModalOpen(true);
    };

    const closeVotingModal = () => {
        setIsVotingModalOpen(false);
        setSelectedVoting(undefined);
    };

    const handleVote = async (votingId: number, selectedOption: string) => {
        const votingService = new VotingService(ICP_CANISTER_ID.VOTING, identity);

        try {
            await votingService.vote(votingId, selectedOption);

            refreshData();

            toast({
                title: "Success!",
                description: "Voted successfully.",
                duration: 2000,
            });
        } catch (error) {
            toast({
                title: "Error!",
                description: "An error occurred while voting.",
                duration: 2000,
            });

            throw error;
        }
    };

    return (
        <>
            <Card>
                <CardHeader>
                    <div className="flex items-center justify-between">
                        <CardTitle>Votations</CardTitle>
                        <Button onClick={() => navigate(`/daos/${dao.principal.toText()}/votings/create`)}>Create votation</Button>
                    </div>
                    <CardDescription>Review and manage all voting processes</CardDescription>
                </CardHeader>
                <CardContent>
                    {dao.pools.length > 0 ? (
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <TableHead>Title</TableHead>
                                    <TableHead>Status</TableHead>
                                    <TableHead>Ends</TableHead>
                                    <TableHead>Participation</TableHead>
                                    <TableHead>Actions</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                {dao.pools.map((pool) => <VotationRow key={pool.id} pool={pool} canUserVote={canUserVote(pool)} openVotingModal={openVotingModal} />)}
                            </TableBody>
                        </Table>
                    ) : (
                        <div className="text-center py-8 text-slate-500">
                            <File className="h-12 w-12 mx-auto mb-4 text-slate-300" />
                            <h3 className="font-medium mb-1">No votations yet</h3>
                            <p>Create a new votation to get started.</p>
                            <Button className="mt-4" onClick={() => navigate(`/daos/${dao.principal.toText()}/votings/create`)}>Create votation</Button>
                        </div>
                    )}
                </CardContent>
            </Card>

            <VotingModal
                userPrincipal={userPrincipal}
                voting={selectedVoting}
                isOpen={isVotingModalOpen}
                onClose={closeVotingModal}
                onVote={handleVote}
            />
        </>
    )
}