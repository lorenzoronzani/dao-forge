import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { List } from "lucide-react";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { capitalizeFirstLetter } from "@/utils/string";

type VotingOptionsCardProps = {
    options: string[];
    areCustomOptions: boolean;
    onValueChange: (field: string, value: string[] | boolean) => void;
}

export const VotingOptionsCard = ({ options, areCustomOptions, onValueChange }: VotingOptionsCardProps) => {
    const addOption = () => {
        onValueChange('options', [...options, '']);
    };

    const updateOption = (index: number, value: string) => {
        const updatedOptions = [...options];
        updatedOptions[index] = capitalizeFirstLetter(value);
        onValueChange('options', updatedOptions);
    };

    const removeOption = (index: number) => {
        const updatedOptions = [...options];
        updatedOptions.splice(index, 1);
        onValueChange('options', updatedOptions);
    };

    return (
        <Card>
            <CardHeader>
                <CardTitle className="flex items-center gap-2">
                    <List className="h-5 w-5" />
                    Voting Options
                </CardTitle>
                <CardDescription>Add at least two options for voters to choose from</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
                <div className="flex items-center justify-between p-4 border rounded-lg">
                    <div className="space-y-0.5">
                        <Label htmlFor="voting-type" className="text-base">
                            {areCustomOptions ? 'Custom Voting' : 'Standard Voting'}
                        </Label>
                        <div className="text-sm text-slate-500">
                            {areCustomOptions
                                ? 'Define your own voting options'
                                : 'Simple Accept/Reject voting for executing proposals'
                            }
                        </div>
                    </div>
                    <Switch
                        id="voting-type"
                        checked={areCustomOptions}
                        onCheckedChange={(checked) => onValueChange('areCustomOptions', checked)}
                    />
                </div>

                {options.map((option, index) => (
                    <div key={index} className="flex items-center gap-2">
                        <Input
                            value={option}
                            onChange={(e) => updateOption(index, e.target.value)}
                            placeholder={`Option ${index + 1}`}
                            required
                            disabled={!areCustomOptions}
                        />
                        {options.length > 2 && (
                            <Button
                                type="button"
                                variant="ghost"
                                size="icon"
                                onClick={() => removeOption(index)}
                            >
                                <span className="sr-only">Remove</span>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="h-4 w-4"><path d="M18 6 6 18"></path><path d="m6 6 12 12"></path></svg>
                            </Button>
                        )}
                    </div>
                ))}

                {areCustomOptions && <Button
                    type="button"
                    variant="outline"
                    className="w-full"
                    onClick={addOption}
                >
                    Add Option
                </Button>}
            </CardContent>
        </Card>
    );
}

export default VotingOptionsCard;