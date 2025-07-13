import { Label } from "@/components/ui/label";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import { MethodSignature, Parameter } from "@/services/canisterAnalyzerService";
import { ActionFormData } from "@/components/forms/VotingForm";
import { VOTING_FORM } from "@/constants/placeholders";

interface DynamicParamsSectionProps {
    action: ActionFormData;
    onValueChange: (field: string, value: unknown) => void;
    methodSig: MethodSignature;
    areCustomOptions: boolean;
}

export const DynamicParamsSection = ({ action, onValueChange, methodSig, areCustomOptions }: DynamicParamsSectionProps) => {
    const handleParameterChange = (index: number, value: string, name?: string) => {
        if (name) {
            const parsedArgs = action.args[index] ? JSON.parse(action.args[index]) : {};
            parsedArgs[name] = value;
            action.args[index] = JSON.stringify(parsedArgs);
        } else {
            action.args[index] = value;
        }

        onValueChange('action', action);
    };

    const getRecordParameterValue = (index: number, name?: string) => {
        if (!name) {
            return action.args[index];
        }

        const parsedArgs = action.args[index] ? JSON.parse(action.args[index]) : {};
        return parsedArgs[name];
    }

    const isUsingWinningOption = (index: number, name?: string) => {
        if (name) {
            const parsedArgs = action.args[index] ? JSON.parse(action.args[index]) : {};
            return parsedArgs[name] === VOTING_FORM.WINNING_OPTION;
        }

        return action.args[index] === VOTING_FORM.WINNING_OPTION;
    }

    const getParamRow = (param: Parameter, index: number) => {
        const paramKey = param.type;
        const inputType = 'text';

        return (
            <>
                <Label htmlFor={paramKey} className="text-sm flex items-center">
                    <Badge variant="outline" className="text-xs whitespace-nowrap">
                        {`${param?.name ?? param.type}`}
                    </Badge>
                </Label>

                <Input
                    id={paramKey}
                    type={inputType}
                    value={getRecordParameterValue(index, param.name) || ''}
                    onChange={(e) => handleParameterChange(index, e.target.value, param?.name)}
                    placeholder="Enter value..."
                    required
                    disabled={isUsingWinningOption(index, param?.name)}
                />

                {areCustomOptions && (
                    <div className="flex items-center gap-2 p-2 bg-blue-50 rounded border">
                        <input
                            type="checkbox"
                            id={`winning-${paramKey}`}
                            checked={isUsingWinningOption(index, param?.name)}
                            onChange={(e) => handleParameterChange(index, e.target.checked ? VOTING_FORM.WINNING_OPTION : '', param?.name)}
                            className="rounded border-gray-300"
                        />
                        <Label htmlFor={`winning-${paramKey}`} className="text-xs text-blue-800">
                            Use winning option
                        </Label>
                    </div>
                )}
            </>
        );
    }

    const getIsUsingWinningOptionRow = (index: number, param: Parameter, fieldIndex?: number) => {
        const key = fieldIndex ? `${index}-${fieldIndex}` : index;

        return (
            isUsingWinningOption(index, param?.name) && <div key={`winning-${key}`} className="p-3 bg-green-50 border border-green-200 rounded">
                <div className="flex items-center gap-2 mb-2">
                    <span className="text-xs text-green-700 font-medium">Winning option will be used:</span>
                </div>
                <div className="text-xs text-green-600">
                    The winning option from the vote will be passed as the "{param?.name ?? 'param'}: {param.type}" parameter.
                </div>
            </div>
        )
    }

    return (
        <div className="space-y-3">
            <Label className="text-base">Parameters</Label>
            <div className="grid gap-2" style={{ gridTemplateColumns: `auto 1fr ${areCustomOptions ? 'auto' : ''}` }}>
                {/* Data rows */}
                {methodSig.parameters.map((param, index) => {
                    if (param.type === 'record') {
                        return param.fields?.map((field) => getParamRow(field, index));
                    }

                    return getParamRow(param, index);
                })}
            </div>

            {methodSig.parameters.map((param, index) => {
                if (param.type === 'record') {
                    return param.fields?.map((field, fieldIndex) => getIsUsingWinningOptionRow(index, field, fieldIndex));
                }

                return getIsUsingWinningOptionRow(index, param);
            })}
        </div>
    );
}

export default DynamicParamsSection;
