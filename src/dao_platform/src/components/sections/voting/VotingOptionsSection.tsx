import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Label } from "@/components/ui/label";
import { Progress } from "@/components/ui/progress";
import { Voting } from "@/models/entities/Voting";
import { capitalizeFirstLetter } from "@/utils/string";

interface VotingOptionsSectionProps {
    voting: Voting;
    canVote: boolean;
    selectedOption: string;
    setSelectedOption: (option: string) => void;
}

export const VotingOptionsSection = ({
    voting,
    canVote,
    selectedOption,
    setSelectedOption,
}: VotingOptionsSectionProps) => {
    const totalVotes = Array.from(voting.result.values()).reduce((sum, count) => sum + count, 0);

    const getVotePercentage = (option: string): number => {
        if (totalVotes === 0) return 0;
        const votes = voting.result.get(option) || 0;
        return (votes / totalVotes) * 100;
    };

    const options = Array.from(voting.options.keys()).map((option) => capitalizeFirstLetter(option)).sort();

    return (
        <div className="space-y-4">
            <h4 className="font-medium">
                {canVote ? 'Select your choice:' : 'Results:'}
            </h4>

            {canVote ? (
                <RadioGroup value={selectedOption} onValueChange={setSelectedOption}>
                    {options.map((option) => (
                        <div key={option} className="flex items-center space-x-2">
                            <RadioGroupItem value={option} id={option} />
                            <Label htmlFor={option} className="flex-1">{option}</Label>
                        </div>
                    ))}
                </RadioGroup>
            ) : (
                <div className="space-y-3">
                    {options.map((option) => {
                        const votes = voting.result.get(option) || 0;
                        const percentage = getVotePercentage(option);

                        return (
                            <div key={option} className="space-y-1">
                                <div className="flex justify-between text-sm">
                                    <span>{option}</span>
                                    <span>{votes} votes ({percentage.toFixed(1)}%)</span>
                                </div>
                                <Progress value={percentage} className="h-2" />
                            </div>
                        );
                    })}
                </div>
            )}
        </div>
    );
};