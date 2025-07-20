import React, { useState } from 'react';
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Principal } from '@dfinity/principal';
import { Voting, VotingState } from '@/models/entities/Voting';
import { VotationStatusBadge } from '@/components/badge/VotationStatusBadge';
import VotingInfoSection from '@/components/sections/voting/VotingInfoSection';
import UserVoteSection from '@/components/sections/voting/UserVoteSection';
import { VotingOptionsSection } from '@/components/sections/voting/VotingOptionsSection';
import { Loader2 } from 'lucide-react';
import { DisplayedActionSection } from '../sections/voting/action/DisplayedActionSection';
import { NoActionMessage } from '../sections/voting/action/NoActionMessage';
import { DisplayedNotificationSection } from '../sections/voting/notification/DisplayedNotificationSection';

interface VotingModalProps {
    userPrincipal: Principal;
    voting: Voting | undefined;
    isOpen: boolean;
    onClose: () => void;
    onVote: (votingId: number, selectedOption: string) => Promise<void>;
}

export const VotingModal = ({
    userPrincipal,
    voting,
    isOpen,
    onClose,
    onVote,
}: VotingModalProps) => {
    const [selectedOption, setSelectedOption] = useState<string>('');
    const [isSubmitting, setIsSubmitting] = useState(false);

    if (!voting) return null;

    const userVote = voting.votersCast.get(userPrincipal.toText());
    const canVote = voting.state === VotingState.Open && !userVote && voting.votersWhitelist.some(p => p.toText() === userPrincipal.toText());

    const handleClose = () => {
        setSelectedOption('');
        onClose();
    };

    const handleVote = async () => {
        setIsSubmitting(true);

        try {
            await onVote(voting.id, selectedOption);

            handleClose();
        } catch (error) {
            console.error('Error voting:', error);
        } finally {
            setIsSubmitting(false);
        }
    };

    const getWinningOption = (): string => {
        const maxVotes = voting.result.values().reduce((max, votes) => Math.max(max, votes), 0);
        const winningOption = Array.from(voting.result.entries()).find(([_, votes]) => votes === maxVotes)?.[0];

        return winningOption ?? Array.from(voting.options.keys())[0];
    };

    const getActionSection = () => {
        const userSelection = userVote || selectedOption;
        const selectedAction = voting.options.get(userSelection);

        if (!canVote && !userSelection) {
            const actionToDisplay = voting.options.get(getWinningOption());
            return <DisplayedActionSection action={actionToDisplay!} />;
        }

        if (selectedAction) {
            return <DisplayedActionSection action={selectedAction} />;
        } else {
            if (userSelection) {
                return <NoActionMessage message="No action is configured for this option." />;
            } else {
                return <NoActionMessage message="Select an option to see the action details." />;
            }
        }
    };

    const getNotificationSection = () => {
        if (!voting.notification) return null;

        const optionToCheck = canVote ? selectedOption : (userVote || getWinningOption());

        if (optionToCheck && optionToCheck !== 'Reject') {
            return <DisplayedNotificationSection notification={voting.notification} />;
        }

        return null;
    };

    return (
        <Dialog open={isOpen} onOpenChange={handleClose}>
            <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
                <DialogHeader>
                    <div className="flex items-start justify-between pr-6">
                        <div className="flex-1 pr-4">
                            <DialogTitle className="text-lg font-semibold">{voting.title}</DialogTitle>
                            <DialogDescription className="mt-1">
                                {voting.description}
                            </DialogDescription>
                        </div>
                        <VotationStatusBadge status={voting.state} />
                    </div>
                </DialogHeader>

                <div className="space-y-6">
                    <VotingInfoSection voting={voting} />

                    {userVote && (
                        <UserVoteSection userVote={userVote} />
                    )}

                    {getActionSection()}

                    {getNotificationSection()}

                    <VotingOptionsSection
                        voting={voting}
                        canVote={canVote}
                        selectedOption={selectedOption}
                        setSelectedOption={setSelectedOption}
                    />
                </div>

                <DialogFooter className="flex justify-between">
                    <div className="flex gap-2">
                        <Button variant="outline" onClick={handleClose}>
                            Close
                        </Button>
                        {canVote && (
                            <Button
                                onClick={handleVote}
                                disabled={!selectedOption || isSubmitting}
                            >
                                {isSubmitting ? <>
                                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                                    Submitting...
                                </> : 'Cast Vote'}
                            </Button>
                        )}
                    </div>
                </DialogFooter>
            </DialogContent >
        </Dialog >
    );
};