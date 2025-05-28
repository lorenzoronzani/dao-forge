import { Label } from "@/components/ui/label";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import { MethodSignature } from "@/services/canisterAnalyzerService";
import { ActionFormData } from "@/components/forms/VotingForm";
import { VOTING_FORM } from "@/constants/placeholders";

interface DynamicParamsSectionProps {
    action: ActionFormData;
    onValueChange: (field: string, value: unknown) => void;
    methodSig: MethodSignature;
    areCustomOptions: boolean;
}

export const DynamicParamsSection = ({ action, onValueChange, methodSig, areCustomOptions }: DynamicParamsSectionProps) => {
    const handleParameterChange = (index: number, value: string) => {
        action.args[index] = value;
        onValueChange('action', action);
    };

    const isUsingWinningOption = (index: number) => {
        return action.args[index] === VOTING_FORM.WINNING_OPTION;
    }


    return (
        <div className="space-y-3">
            <Label className="text-base">Parameters</Label>
            {methodSig.parametersType.map((param, index) => {
                const paramKey = param;
                const inputType = 'text';

                return (
                    <div key={index} className="space-y-1">
                        <div className="flex items-center gap-2">
                            <Label htmlFor={paramKey} className="text-sm">
                                <Badge variant="outline" className="ml-2 text-xs">
                                    {param}
                                </Badge>
                            </Label>

                            <Input
                                id={paramKey}
                                type={inputType}
                                value={action.args[index] || ''}
                                onChange={(e) => handleParameterChange(index, e.target.value)}
                                placeholder="Enter value..."
                                required
                                disabled={isUsingWinningOption(index)}
                            />

                            {areCustomOptions && <div className="flex items-center gap-2 p-2 bg-blue-50 rounded border">
                                <input
                                    type="checkbox"
                                    id={`winning-${paramKey}`}
                                    checked={isUsingWinningOption(index)}
                                    onChange={(e) => handleParameterChange(index, e.target.checked ? VOTING_FORM.WINNING_OPTION : '')}
                                    className="rounded border-gray-300"
                                />
                                <Label htmlFor={`winning-${paramKey}`} className="text-xs text-blue-800">
                                    Use winning option as parameter value
                                </Label>
                            </div>}
                        </div>

                        {isUsingWinningOption(index) && (
                            <div className="p-3 bg-green-50 border border-green-200 rounded">
                                <div className="flex items-center gap-2 mb-2">
                                    <span className="text-xs text-green-700 font-medium">Winning option will be used:</span>
                                </div>
                                <div className="text-xs text-green-600">
                                    The winning option from the vote will be passed as the "{param}" parameter.
                                </div>
                            </div>
                        )}

                    </div>
                );
            })}
        </div>
    );
}

export default DynamicParamsSection;
